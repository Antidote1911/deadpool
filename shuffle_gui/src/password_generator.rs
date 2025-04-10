use deadpool::*;

pub struct ComplexPasswordGenerator {
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_logograms: bool,
}

impl ComplexPasswordGenerator {
    pub fn new(include_uppercase: bool, include_lowercase: bool, include_numbers: bool, include_logograms: bool) -> Self {
        Self {
            include_uppercase,
            include_lowercase,
            include_numbers,
            include_logograms,
        }
    }
    pub fn generate_password(&self, length: usize) -> String {

        let mut pool = Pool::new();

        if self.include_uppercase {
            pool.extend_from_uppercase();
        }
        if self.include_lowercase {
            pool.extend_from_lowercase();
        }
        if self.include_numbers {
            pool.extend_from_digits();
        }
        if self.include_logograms {
            pool.extend_from_logograms()
        }
        pool.generate(length).unwrap()
    }
}