use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;

mod errors;
use errors::PasswordError;

/// The version of the application, retrieved from the environment variable `CARGO_PKG_VERSION`.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version of the application as a static string.
pub const fn getversion() -> &'static str {
    APP_VERSION
}

/// Character sets used for password generation.
pub struct CharacterSets {
    pub lowercase: &'static str,
    pub uppercase: &'static str,
    pub digits: &'static str,
    pub braces: &'static str,
    pub punctuation: &'static str,
    pub quotes: &'static str,
    pub dashes: &'static str,
    pub math: &'static str,
    pub logograms: &'static str,
}

/// Default character sets for password generation.
pub const DEFAULT_CHARSETS: CharacterSets = CharacterSets {
    lowercase: "abcdefghijklmnopqrstuvwxyz",
    uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    digits: "0123456789",
    braces: "()[]{}",
    punctuation: ".,:;",
    quotes: "\"'",
    dashes: "-/\\_|",
    math: "!*+<=>?",
    logograms: "#$%&@^`~",
};

/// Structure to hold and extend character sets for password generation.
pub struct Pool {
    characters: String,
    selected_sets: Vec<&'static str>,
    custom_string: Option<String>,
    excluded_chars: Option<String>,
}

impl Pool {
    /// Creates a new, empty pool.
    pub fn new() -> Self {
        Pool {
            characters: String::new(),
            selected_sets: Vec::new(),
            custom_string: None,
            excluded_chars: None,
        }
    }

    fn extend_with_charset(&mut self, charset: &'static str) {
        // Silently ignore duplicate set additions to keep distribution fair.
        if !self.selected_sets.contains(&charset) {
            self.characters.push_str(charset);
            self.selected_sets.push(charset);
        }
    }

    pub fn extend_from_lowercase(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.lowercase); }
    pub fn extend_from_uppercase(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.uppercase); }
    pub fn extend_from_digits(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.digits); }
    pub fn extend_from_braces(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.braces); }
    pub fn extend_from_punctuation(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.punctuation); }
    pub fn extend_from_quotes(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.quotes); }
    pub fn extend_from_dashes(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.dashes); }
    pub fn extend_from_math(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.math); }
    pub fn extend_from_logograms(&mut self) { self.extend_with_charset(DEFAULT_CHARSETS.logograms); }

    /// Sets the excluded characters for the pool.
    pub fn exclude_chars(&mut self, excluded: &str) {
        self.excluded_chars = Some(excluded.to_string());
    }

    /// Extends the pool with a custom string, replacing excluded characters with random authorized ones.
    pub fn extend_from_string(&mut self, custom_string: &str) -> Result<(), PasswordError> {
        let mut rng = Hc128Rng::from_os_rng();
        let excluded_chars = self.excluded_chars.as_deref().unwrap_or("");

        let available: Vec<char> = self.characters.chars()
            .filter(|c| !excluded_chars.contains(*c))
            .collect();

        let extended_string: Result<String, PasswordError> = custom_string.chars()
            .map(|ch| {
                if excluded_chars.contains(ch) {
                    if available.is_empty() {
                        Err(PasswordError::NoAvailableCharacters)
                    } else {
                        let idx = rng.random_range(0..available.len());
                        Ok(available[idx])
                    }
                } else {
                    Ok(ch)
                }
            })
            .collect();

        let extended_string = extended_string?;
        self.characters.push_str(&extended_string);
        self.custom_string = Some(extended_string);
        Ok(())
    }

    /// Generates a password of the specified length from the pool.
    pub fn generate(&self, length: usize) -> Result<String, PasswordError> {
        if length == 0 {
            return Err(PasswordError::ZeroLengthPassword);
        }

        if self.is_empty() {
            return Err(PasswordError::EmptyCharacterPool);
        }

        let excluded_chars = self.excluded_chars.as_deref().unwrap_or("");

        // Each selected set contributes one mandatory character; custom string adds one more.
        let min_length = self.selected_sets.len()
            + self.custom_string.as_ref().map_or(0, |_| 1);
        if length < min_length {
            return Err(PasswordError::LengthTooShort { requested: length, minimum: min_length });
        }

        let mut rng = Hc128Rng::from_os_rng();
        let mut password: Vec<char> = Vec::with_capacity(length);

        // Guarantee at least one character from each selected set.
        for set in &self.selected_sets {
            let filtered: Vec<char> = set.chars()
                .filter(|c| !excluded_chars.contains(*c))
                .collect();
            if filtered.is_empty() {
                return Err(PasswordError::EmptyCharacterPool);
            }
            password.push(filtered[rng.random_range(0..filtered.len())]);
        }

        // Guarantee at least one character from the custom string.
        if let Some(custom_string) = &self.custom_string {
            let filtered: Vec<char> = custom_string.chars()
                .filter(|c| !excluded_chars.contains(*c))
                .collect();
            if !filtered.is_empty() {
                password.push(filtered[rng.random_range(0..filtered.len())]);
            }
        }

        // Pre-calculate once before the fill loop — avoids O(n²) recomputation.
        let filtered_chars: Vec<char> = self.characters.chars()
            .filter(|c| !excluded_chars.contains(*c))
            .collect();
        if filtered_chars.is_empty() {
            return Err(PasswordError::NoAvailableCharacters);
        }

        while password.len() < length {
            password.push(filtered_chars[rng.random_range(0..filtered_chars.len())]);
        }

        password.shuffle(&mut rng);
        Ok(password.iter().collect())
    }

    /// Checks if the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.characters.is_empty()
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test;
