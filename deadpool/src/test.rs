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
        assert!(password
            .chars()
            .all(|ch| !pool.excluded_chars.as_ref().unwrap().contains(ch)));
    }

    #[test]
    fn test_generate_with_custom_string() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.extend_from_digits();
        pool.exclude_chars("aeiou");
        pool.extend_from_string("hello123").unwrap();

        let password = pool.generate(10);
        assert!(password.is_ok());
        let password = password.unwrap();
        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|ch| pool.characters.contains(ch)));
    }

    #[test]
    fn test_generate_with_only_excluded_characters() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.exclude_chars("abcdefghijklmnopqrstuvwxyz");
        let result = pool.extend_from_string("test");
        assert!(matches!(result, Err(PasswordError::NoAvailableCharacters)));
    }

    #[test]
    fn test_no_duplicate_sets() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.extend_from_lowercase(); // duplicate — should be ignored
        assert_eq!(pool.selected_sets.len(), 1);
        assert_eq!(pool.characters.len(), DEFAULT_CHARSETS.lowercase.len());
    }

    #[test]
    fn test_length_too_short() {
        let mut pool = Pool::new();
        pool.extend_from_lowercase();
        pool.extend_from_uppercase();
        pool.extend_from_digits(); // 3 sets → minimum length = 3
        let result = pool.generate(2);
        assert!(matches!(result, Err(PasswordError::LengthTooShort { .. })));
    }

    #[test]
    fn test_default_pool_is_empty() {
        let pool = Pool::default();
        assert!(pool.is_empty());
    }
}
