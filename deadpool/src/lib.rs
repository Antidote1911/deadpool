use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_hc::Hc128Rng;
use rand_isaac::Isaac64Rng;

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
    excluded_chars: Option<String>,  // Field for excluded characters
}

impl Pool {
    /// Creates a new, empty pool.
    pub fn new() -> Self {
        Pool {
            characters: String::new(),
            selected_sets: Vec::new(),
            custom_string: None,
            excluded_chars: None,  // Initialize excluded_chars
        }
    }

    /// Extends the pool with a given character set.
    fn extend_with_charset(&mut self, charset: &'static str) {
        self.characters.push_str(charset);
        self.selected_sets.push(charset);
    }

    /// Extends the pool with lowercase characters.
    pub fn extend_from_lowercase(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.lowercase);
    }

    /// Extends the pool with uppercase characters.
    pub fn extend_from_uppercase(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.uppercase);
    }

    /// Extends the pool with digits.
    pub fn extend_from_digits(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.digits);
    }

    /// Extends the pool with braces.
    pub fn extend_from_braces(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.braces);
    }

    /// Extends the pool with punctuation characters.
    pub fn extend_from_punctuation(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.punctuation);
    }

    /// Extends the pool with quotes.
    pub fn extend_from_quotes(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.quotes);
    }

    /// Extends the pool with dashes.
    pub fn extend_from_dashes(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.dashes);
    }

    /// Extends the pool with math characters.
    pub fn extend_from_math(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.math);
    }

    /// Extends the pool with logograms.
    pub fn extend_from_logograms(&mut self) {
        self.extend_with_charset(DEFAULT_CHARSETS.logograms);
    }

    /// Sets the excluded characters for the pool.
    pub fn exclude_chars(&mut self, excluded: &str) {
        self.excluded_chars = Some(excluded.to_string());
    }

    /// Extends the pool with a custom string, replacing excluded characters with random authorized characters.
    pub fn extend_from_string(&mut self, custom_string: &str) {
        let mut isaac_seeder = Isaac64Rng::from_os_rng();
        let mut rng = Hc128Rng::from_rng(&mut isaac_seeder);

        let excluded_chars = self.excluded_chars.as_deref().unwrap_or("");

        let extended_string: String = custom_string.chars()
            .map(|ch| {
                if excluded_chars.contains(ch) {
                    let filtered_chars: String = self.characters.chars()
                        .filter(|c| !excluded_chars.contains(*c))
                        .collect();
                    if filtered_chars.is_empty() {
                        panic!("No available characters to replace excluded ones.");
                    }
                    let idx = rng.random_range(0..filtered_chars.chars().count());
                    filtered_chars.chars().nth(idx).unwrap()
                } else {
                    ch
                }
            })
            .collect();

        self.characters.push_str(&extended_string);
        self.custom_string = Some(extended_string);
    }


    /// Generates a password of the specified length from the pool.
    pub fn generate(&self, length: usize) -> Result<String, PasswordError> {

        if length == 0 {
            return Err(PasswordError::ZeroLengthPassword);
        }

        if self.is_empty() {
            return Err(PasswordError::EmptyCharacterPool);
        }

        let mut isaac_seeder = Isaac64Rng::from_os_rng();
        let mut rng = Hc128Rng::from_rng(&mut isaac_seeder);

        let excluded_chars = self.excluded_chars.as_deref().unwrap_or("");

        let mut password: Vec<char> = Vec::with_capacity(length);

        // Ensure at least one character per selected set
        for set in &self.selected_sets {
            let filtered_set: String = set.chars()
                .filter(|c| !excluded_chars.contains(*c))
                .collect();
            if filtered_set.is_empty() {
                return Err(PasswordError::EmptyCharacterPool); // Empty range for this set
            }
            let idx = rng.random_range(0..filtered_set.chars().count());
            password.push(filtered_set.chars().nth(idx).unwrap());
        }

        if let Some(custom_string) = &self.custom_string {
            let filtered_custom: String = custom_string.chars()
                .filter(|c| !excluded_chars.contains(*c))
                .collect();
            if !filtered_custom.is_empty() {
                let idx = rng.random_range(0..filtered_custom.chars().count());
                password.push(filtered_custom.chars().nth(idx).unwrap());
            }
        }

        while password.len() < length {
            let filtered_chars: String = self.characters.chars()
                .filter(|c| !excluded_chars.contains(*c))
                .collect();
            if filtered_chars.is_empty() {
                return Err(PasswordError::NoAvailableCharacters); // No valid characters available
            }
            let idx = rng.random_range(0..filtered_chars.chars().count());
            password.push(filtered_chars.chars().nth(idx).unwrap());
        }

        password.shuffle(&mut rng);
        Ok(password.iter().collect())
    }

    /// Checks if the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.characters.is_empty()
    }
}

#[cfg(test)]
mod test;
