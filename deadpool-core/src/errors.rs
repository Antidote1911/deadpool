use thiserror::Error;

/// Error type for password generation
#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("password length must be greater than 0")]
    ZeroLengthPassword,
    #[error("character pool must not be empty")]
    EmptyCharacterPool,
    #[error("no available characters to replace excluded ones")]
    NoAvailableCharacters,
    #[error("requested length {requested} is less than the minimum required {minimum} (one character per selected set)")]
    LengthTooShort { requested: usize, minimum: usize },
}
