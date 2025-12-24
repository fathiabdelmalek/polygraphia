use crate::error::PolygraphiaError;
use crate::traits::Cipher;
use crate::utils::math;
use crate::utils::mode::TextMode;

#[derive(Debug, Clone)]
pub struct Affine {
    shift: u8,
    multiplier: u8,
    inv_multiplier: u8,
    mode: TextMode,
}

impl Affine {
    pub fn new(shift: u8, multiplier: u8) -> Result<Self, PolygraphiaError> {
        Self::with_mode(shift, multiplier, TextMode::default())
    }

    pub fn shift(&self) -> u8 {
        self.shift
    }

    pub fn multiplier(&self) -> u8 {
        self.multiplier
    }

    pub fn mode(&self) -> TextMode {
        self.mode
    }

    pub fn set_shift(&mut self, shift: u8) -> Result<(), PolygraphiaError> {
        self.shift = shift % 26;
        Ok(())
    }

    pub fn set_multiplier(&mut self, multiplier: u8) -> Result<(), PolygraphiaError> {
        Self::validate_multiplier(multiplier)?;
        self.multiplier = multiplier;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: TextMode) {
        self.mode = mode;
    }

    fn with_mode(shift: u8, multiplier: u8, mode: TextMode) -> Result<Self, PolygraphiaError> {
        Self::validate_multiplier(multiplier)?;
        let inv_multiplier = math::mod_inverse(multiplier, 26)?;
        Ok(Affine {
            shift: shift % 26,
            multiplier,
            inv_multiplier,
            mode,
        })
    }

    fn validate_multiplier(multiplier: u8) -> Result<(), PolygraphiaError> {
        if !math::are_coprime(multiplier, 26) {
            return Err(PolygraphiaError::InvalidKey(format!(
                "Multiplier {} must be coprime with 26 (gcd = {}). Valid values: 1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25",
                multiplier,
                math::gcd(multiplier, 26)
            )));
        }
        Ok(())
    }

    fn process_char(&self, c: char, encrypt: bool) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }
        let is_uppercase = c.is_ascii_uppercase();
        let base = if is_uppercase { b'A' } else { b'a' };
        let c_lower = c.to_ascii_lowercase();
        let idx = (c_lower as u8) - b'a';
        let processed_idx = if encrypt {
            ((self.multiplier as u16 * idx as u16 + self.shift as u16) % 26) as u8
        } else {
            let shifted = (idx as i16 - self.shift as i16).rem_euclid(26) as u8;
            ((self.inv_multiplier as u16 * shifted as u16) % 26) as u8
        };
        (base + processed_idx) as char
    }

    fn process_text(&self, text: &str, encrypt: bool) -> String {
        match self.mode {
            TextMode::AlphaOnly => text
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .map(|c| self.process_char(c, encrypt))
                .collect(),
            TextMode::PreserveAll => text
                .chars()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        self.process_char(c, encrypt)
                    } else {
                        c
                    }
                })
                .collect(),
        }
    }
}

impl Cipher for Affine {
    fn encrypt(&self, plaintext: &str) -> Result<String, PolygraphiaError> {
        if plaintext.is_empty() {
            return Err(PolygraphiaError::InvalidInput(
                "Empty plaintext".to_string(),
            ));
        }
        let result = self.process_text(plaintext, true);
        if self.mode == TextMode::AlphaOnly && result.is_empty() {
            return Err(PolygraphiaError::InvalidInput(
                "Plaintext must contain at least one alphabetic character".to_string(),
            ));
        }
        Ok(result)
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, PolygraphiaError> {
        if ciphertext.is_empty() {
            return Err(PolygraphiaError::InvalidInput(
                "Empty ciphertext".to_string(),
            ));
        }
        let result = self.process_text(ciphertext, false);
        if self.mode == TextMode::AlphaOnly && result.is_empty() {
            return Err(PolygraphiaError::InvalidInput(
                "ciphertext must have at least one alphabetic character".to_string(),
            ));
        }
        Ok(result)
    }

    fn name(&self) -> &str {
        "affine"
    }
}

impl Drop for Affine {
    fn drop(&mut self) {
        self.shift = 0;
        self.multiplier = 0;
        self.inv_multiplier = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affine_new() {
        let cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.multiplier(), 5);
        assert_eq!(cipher.shift(), 8);
        assert_eq!(cipher.mode(), TextMode::PreserveAll);
    }

    #[test]
    fn test_affine_invalid_multipliers() {
        // Even numbers are not coprime with 26
        assert!(Affine::new(5, 2).is_err());
        assert!(Affine::new(5, 4).is_err());
        assert!(Affine::new(5, 6).is_err());
        assert!(Affine::new(5, 8).is_err());

        // 13 is a factor of 26
        assert!(Affine::new(5, 13).is_err());

        // 26 itself
        assert!(Affine::new(5, 26).is_err());
    }

    #[test]
    fn test_affine_valid_multipliers() {
        // All coprime values with 26
        let valid = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
        for &mult in &valid {
            assert!(
                Affine::new(0, mult).is_ok(),
                "Multiplier {} should be valid",
                mult
            );
        }
    }

    #[test]
    fn test_affine_encrypt_basic() {
        let cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "rclla");
    }

    #[test]
    fn test_affine_decrypt_basic() {
        let cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.decrypt("rclla").unwrap(), "hello");
    }

    #[test]
    fn test_affine_encrypt_decrypt_roundtrip() {
        let cipher = Affine::new(8, 5).unwrap();
        let plaintext = "hello world";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_affine_preserve_all_mode() {
        let cipher = Affine::new(8, 5).unwrap();

        // Numbers preserved
        assert_eq!(cipher.encrypt("hello123").unwrap(), "rclla123");

        // Special characters preserved
        assert_eq!(cipher.encrypt("Hello, World!").unwrap(), "Rclla, Oaplx!");

        // Mixed content
        assert_eq!(cipher.encrypt("Test123!@#").unwrap(), "Zcuz123!@#");
    }

    #[test]
    fn test_affine_alpha_only_mode() {
        let cipher = Affine::with_mode(8, 5, TextMode::AlphaOnly).unwrap();

        // Numbers filtered out
        assert_eq!(cipher.encrypt("hello123").unwrap(), "rclla");

        // Special characters filtered out
        assert_eq!(cipher.encrypt("Hello, World!").unwrap(), "RcllaOaplx");
    }

    #[test]
    fn test_affine_case_preservation() {
        let cipher = Affine::new(8, 5).unwrap();

        assert_eq!(cipher.encrypt("HeLLo").unwrap(), "RcLLa");
        assert_eq!(cipher.decrypt("RcLLa").unwrap(), "HeLLo");

        assert_eq!(cipher.encrypt("ABC").unwrap(), "INS");
        assert_eq!(cipher.encrypt("abc").unwrap(), "ins");
    }

    #[test]
    fn test_affine_all_valid_multipliers() {
        let valid_multipliers = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
        let plaintext = "the quick brown fox";

        for &mult in &valid_multipliers {
            let cipher = Affine::new(mult, 5).unwrap();
            let encrypted = cipher.encrypt(plaintext).unwrap();
            let decrypted = cipher.decrypt(&encrypted).unwrap();

            assert_eq!(
                decrypted, plaintext,
                "Roundtrip failed for multiplier {}",
                mult
            );
        }
    }

    #[test]
    fn test_affine_set_multiplier() {
        let mut cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "rclla");

        cipher.set_multiplier(7).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "fkhhc");

        // Invalid multiplier should error
        assert!(cipher.set_multiplier(2).is_err());
    }

    #[test]
    fn test_affine_set_shift() {
        let mut cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "rclla");

        let _ = cipher.set_shift(3);
        assert_eq!(cipher.encrypt("hello").unwrap(), "mxggv");
    }

    #[test]
    fn test_affine_set_mode() {
        let mut cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.encrypt("hello123").unwrap(), "rclla123");

        cipher.set_mode(TextMode::AlphaOnly);
        assert_eq!(cipher.encrypt("hello123").unwrap(), "rclla");
    }

    #[test]
    fn test_affine_empty_input() {
        let cipher = Affine::new(8, 5).unwrap();
        assert!(cipher.encrypt("").is_err());
        assert!(cipher.decrypt("").is_err());
    }

    #[test]
    fn test_affine_no_alpha_chars_alpha_only() {
        let cipher = Affine::with_mode(8, 5, TextMode::AlphaOnly).unwrap();
        assert!(cipher.encrypt("123!@#").is_err());
    }

    #[test]
    fn test_affine_no_alpha_chars_preserve_all() {
        let cipher = Affine::new(8, 5).unwrap();
        assert_eq!(cipher.encrypt("123!@#").unwrap(), "123!@#");
    }

    #[test]
    fn test_affine_cipher_trait() {
        let cipher: Box<dyn Cipher> = Box::new(Affine::new(8, 5).unwrap());
        assert_eq!(cipher.name(), "affine");
        assert_eq!(cipher.encrypt("hello").unwrap(), "rclla");
    }

    #[test]
    fn test_affine_modulo_behavior() {
        // Test that shift wraps correctly
        let cipher1 = Affine::new(8, 5).unwrap();
        let cipher2 = Affine::new(34, 5).unwrap(); // 34 % 26 = 8

        let text = "test";
        assert_eq!(
            cipher1.encrypt(text).unwrap(),
            cipher2.encrypt(text).unwrap()
        );
    }
}
