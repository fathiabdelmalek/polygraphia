use crate::error::PolygraphiaError;
use crate::traits::Cipher;
use crate::utils::TextMode;

#[derive(Debug, Clone)]
pub struct Caesar {
    shift: u8,
    mode: TextMode,
}

impl Caesar {
    pub fn new(shift: u8) -> Result<Self, PolygraphiaError> {
        Self::with_mode(shift, TextMode::default())
    }

    pub fn shift(&self) -> u8 {
        self.shift
    }

    pub fn mode(&self) -> TextMode {
        self.mode
    }

    pub fn set_shift(&mut self, shift: u8) -> Result<(), PolygraphiaError> {
        self.shift = shift % 26;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: TextMode) {
        self.mode = mode;
    }

    fn with_mode(shift: u8, mode: TextMode) -> Result<Self, PolygraphiaError> {
        Ok(Caesar {
            shift: shift % 26,
            mode,
        })
    }

    fn shift_char(&self, c: char, encrypt: bool) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }
        let is_uppercase = c.is_ascii_uppercase();
        let base = if is_uppercase { b'A' } else { b'a' };
        let c_lower = c.to_ascii_lowercase();
        let idx = (c_lower as u8) - b'a';
        let shift = if encrypt {
            self.shift as i8
        } else {
            -(self.shift as i8)
        };
        let shifted = (idx as i8 + shift).rem_euclid(26) as u8;
        (base + shifted) as char
    }

    fn process_text(&self, text: &str, encrypt: bool) -> String {
        match self.mode {
            TextMode::AlphaOnly => text
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .map(|c| self.shift_char(c, encrypt))
                .collect(),
            TextMode::PreserveAll => text
                .chars()
                .map(|c| {
                    if c.is_ascii_alphabetic() {
                        self.shift_char(c, encrypt)
                    } else {
                        c
                    }
                })
                .collect(),
        }
    }
}

impl Cipher for Caesar {
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
        "caesar"
    }
}

impl Drop for Caesar {
    fn drop(&mut self) {
        self.shift = 0;
    }
}

// ==================== Tests ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_new() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.shift(), 3);
        assert_eq!(cipher.mode(), TextMode::PreserveAll);
    }

    #[test]
    fn test_caesar_with_mode() {
        let cipher = Caesar::with_mode(3, TextMode::AlphaOnly).unwrap();
        assert_eq!(cipher.mode(), TextMode::AlphaOnly);
    }

    #[test]
    fn test_caesar_new_with_modulo() {
        let cipher = Caesar::new(29).unwrap();
        assert_eq!(cipher.shift(), 3); // 29 % 26 = 3
    }

    #[test]
    fn test_caesar_zero_shift() {
        let cipher = Caesar::new(0).unwrap();
        assert_eq!(cipher.shift(), 0);
        // Zero shift means no encryption
        assert_eq!(cipher.encrypt("hello").unwrap(), "hello");
    }

    #[test]
    fn test_caesar_encrypt_preserve_all() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");
        assert_eq!(cipher.encrypt("HELLO").unwrap(), "KHOOR");
        assert_eq!(cipher.encrypt("Hello World!").unwrap(), "Khoor Zruog!");
        assert_eq!(cipher.encrypt("hello123").unwrap(), "khoor123");
        assert_eq!(cipher.encrypt("Test123!@#").unwrap(), "Whvw123!@#");
    }

    #[test]
    fn test_caesar_encrypt_alpha_only() {
        let cipher = Caesar::with_mode(3, TextMode::AlphaOnly).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");
        assert_eq!(cipher.encrypt("Hello World!").unwrap(), "KhoorZruog");
        assert_eq!(cipher.encrypt("hello123").unwrap(), "khoor");
        assert_eq!(cipher.encrypt("Test123!@#").unwrap(), "Whvw");
    }

    #[test]
    fn test_caesar_decrypt_preserve_all() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.decrypt("khoor").unwrap(), "hello");
        assert_eq!(cipher.decrypt("KHOOR").unwrap(), "HELLO");
        assert_eq!(cipher.decrypt("Khoor Zruog!").unwrap(), "Hello World!");
        assert_eq!(cipher.decrypt("khoor123").unwrap(), "hello123");
    }

    #[test]
    fn test_caesar_decrypt_alpha_only() {
        let cipher = Caesar::with_mode(3, TextMode::AlphaOnly).unwrap();
        assert_eq!(cipher.decrypt("khoor").unwrap(), "hello");
        assert_eq!(cipher.decrypt("KhoorZruog").unwrap(), "HelloWorld");
    }

    #[test]
    fn test_caesar_encrypt_decrypt_roundtrip_preserve_all() {
        let cipher = Caesar::new(7).unwrap();
        let plaintext = "The Quick Brown Fox Jumps Over The Lazy Dog 123!";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(
            decrypted,
            "The Quick Brown Fox Jumps Over The Lazy Dog 123!"
        );

        // Verify roundtrip
        let cipher2 = Caesar::new(7).unwrap();
        assert_eq!(
            cipher2.decrypt(&encrypted).unwrap(),
            "The Quick Brown Fox Jumps Over The Lazy Dog 123!"
        );
    }

    #[test]
    fn test_caesar_encrypt_decrypt_roundtrip_alpha_only() {
        let cipher = Caesar::with_mode(7, TextMode::AlphaOnly).unwrap();
        let plaintext = "The Quick Brown Fox 123!";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(encrypted, "AolXbpjrIyvduMve");
        assert_eq!(decrypted, "TheQuickBrownFox");
    }

    #[test]
    fn test_caesar_wrap_around() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("xyz").unwrap(), "abc");
        assert_eq!(cipher.decrypt("abc").unwrap(), "xyz");
        assert_eq!(cipher.encrypt("XYZ").unwrap(), "ABC");
    }

    #[test]
    fn test_caesar_case_preservation() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("HeLLo").unwrap(), "KhOOr");
        assert_eq!(cipher.decrypt("KhOOr").unwrap(), "HeLLo");
    }

    #[test]
    fn test_caesar_set_shift() {
        let mut cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");

        let _ = cipher.set_shift(5);
        assert_eq!(cipher.encrypt("hello").unwrap(), "mjqqt");
    }

    #[test]
    fn test_caesar_set_mode() {
        let mut cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("hello123").unwrap(), "khoor123");

        cipher.set_mode(TextMode::AlphaOnly);
        assert_eq!(cipher.encrypt("hello123").unwrap(), "khoor");
    }

    #[test]
    fn test_caesar_empty_input() {
        let cipher = Caesar::new(3).unwrap();
        assert!(cipher.encrypt("").is_err());
        assert!(cipher.decrypt("").is_err());
    }

    #[test]
    fn test_caesar_no_alpha_characters_alpha_only() {
        let cipher = Caesar::with_mode(3, TextMode::AlphaOnly).unwrap();
        assert!(cipher.encrypt("123!@#").is_err());
    }

    #[test]
    fn test_caesar_no_alpha_characters_preserve_all() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("123!@#").unwrap(), "123!@#");
    }

    #[test]
    fn test_cipher_trait() {
        let cipher: Caesar = Box::new(Caesar::new(3)).unwrap();
        assert_eq!(cipher.name(), "caesar");
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");
    }

    #[test]
    fn test_numbers_and_symbols() {
        let cipher = Caesar::new(5).unwrap();

        // Test various number combinations
        assert_eq!(cipher.encrypt("abc123def").unwrap(), "fgh123ijk");
        assert_eq!(cipher.decrypt("fgh123ijk").unwrap(), "abc123def");

        // Test symbols
        assert_eq!(
            cipher.encrypt("hello@world.com").unwrap(),
            "mjqqt@btwqi.htr"
        );

        // Test mixed content
        assert_eq!(cipher.encrypt("Password123!").unwrap(), "Ufxxbtwi123!");
    }
}
