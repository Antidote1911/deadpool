#[cfg(test)]
mod tests {
      use crate::*;

    #[test]
    fn test_generate_valid_password() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.extend_from_digits();

        let password = pool.generate(10);
        assert!(password.is_ok());
        let password = password.unwrap();
        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|ch| pool.characters.contains(ch)));
    }

    #[test]
    fn test_generate_empty_pool() {
        let pool = Pool::new();
        let result = pool.generate(10);
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                PasswordError::EmptyCharacterPool => (),
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn test_generate_zero_length_password() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        let result = pool.generate(0);
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                PasswordError::ZeroLengthPassword => (),
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn test_generate_with_excluded_characters() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.exclude_chars("aeiou");

        let password = pool.generate(10);
        assert!(password.is_ok());
        let password = password.unwrap();
        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|ch| !pool.excluded_chars.as_ref().unwrap().contains(ch)));
    }

    #[test]
    fn test_generate_with_custom_string() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.extend_from_digits();
        pool.exclude_chars("aeiou");
        pool.extend_from_string("hello123");

        let password = pool.generate(10);
        assert!(password.is_ok());
        let password = password.unwrap();
        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|ch| pool.characters.contains(ch)));
    }

    #[test]
    #[should_panic(expected = "No available characters to replace excluded ones.")]
    fn test_generate_with_only_excluded_characters() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.exclude_chars("abcdefghijklmnopqrstuvwxyz");
        // This will trigger a panic since no characters are left to generate a password.
        pool.extend_from_string("test");
    }

}