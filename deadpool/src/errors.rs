use thiserror::Error;

/// Error type for password generation
#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("password length must be greater than 0")]
    ZeroLengthPassword,
    #[error("character pool must not be empty")]
    EmptyCharacterPool,
    #[error("No available characters to replace excluded ones")]
    NoAvailableCharacters,
}
