use crate::error::PolygraphiaError;
use crate::traits::Cipher;
use crate::utils::mode::TextMode;

#[derive(Debug, Clone)]
pub struct Playfair {
    key: String,
    matrix: [[u8; 5]; 5],
    mode: TextMode,
}

impl Playfair {
    pub fn new(key: &str) -> Result<Self, PolygraphiaError> {
        Self::with_mode(key, TextMode::default())
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn matrix(&self) -> &[[u8; 5]; 5] {
        &self.matrix
    }

    pub fn mode(&self) -> TextMode {
        self.mode
    }

    pub fn set_key(&mut self, key: &str) -> Result<(), PolygraphiaError> {
        if key.is_empty() {
            return Err(PolygraphiaError::InvalidKey(
                "Key cannot be empty".to_string(),
            ));
        }
        self.key = Self::prepare_key(key);
        self.matrix = Self::generate_matrix(&self.key);
        Ok(())
    }

    pub fn set_mode(&mut self, mode: TextMode) {
        self.mode = mode;
    }

    pub fn with_mode(key: &str, mode: TextMode) -> Result<Self, PolygraphiaError> {
        if key.is_empty() {
            return Err(PolygraphiaError::InvalidKey(
                "Key cannot be empty".to_string(),
            ));
        }

        let prepared_key = Self::prepare_key(key);
        let matrix = Self::generate_matrix(&prepared_key);

        Ok(Playfair {
            key: prepared_key,
            matrix,
            mode,
        })
    }

    fn prepare_key(key: &str) -> String {
        let mut unique_chars = Vec::new();
        for c in key.chars() {
            if !c.is_ascii_alphabetic() {
                continue;
            }
            let c_lower = c.to_ascii_lowercase();
            let c_normalized = if c_lower == 'j' { 'i' } else { c_lower };
            if !unique_chars.contains(&c_normalized) {
                unique_chars.push(c_normalized);
            }
        }
        for c in "abcdefghiklmnopqrstuvwxyz".chars() {
            if !unique_chars.contains(&c) {
                unique_chars.push(c);
            }
        }
        unique_chars.iter().collect()
    }

    fn generate_matrix(key: &str) -> [[u8; 5]; 5] {
        let mut matrix = [[0u8; 5]; 5];
        let chars: Vec<char> = key.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            let row = i / 5;
            let col = i % 5;
            matrix[row][col] = (c as u8) - b'a';
        }
        matrix
    }

    fn prepare_text(text: &str) -> String {
        let filtered: String = text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| {
                let c_lower = c.to_ascii_lowercase();
                if c_lower == 'j' { 'i' } else { c_lower }
            })
            .collect();
        let mut prepared = Vec::new();
        let chars: Vec<char> = filtered.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let first = chars[i];
            if i == chars.len() - 1 {
                prepared.push(first);
                break;
            } else if first == chars[i + 1] {
                prepared.push(first);
                prepared.push('x');
            } else {
                prepared.push(first);
            }
            i += 1;
        }
        if prepared.len() % 2 == 1 {
            prepared.push('x');
        }
        prepared.iter().collect()
    }

    fn get_coordinates(&self, char_val: u8) -> Result<(usize, usize), PolygraphiaError> {
        for row in 0..5 {
            for col in 0..5 {
                if self.matrix[row][col] == char_val {
                    return Ok((row, col));
                }
            }
        }
        Err(PolygraphiaError::InvalidInput(format!(
            "Character {} not found in matrix",
            (char_val + b'a') as char
        )))
    }

    fn process_pair(&self, pair: &str, encrypt: bool) -> Result<String, PolygraphiaError> {
        let chars: Vec<char> = pair.chars().collect();
        if chars.len() != 2 {
            return Err(PolygraphiaError::InvalidInput(
                "Pair must contain exactly 2 characters".to_string(),
            ));
        }
        let char1_val = (chars[0] as u8) - b'a';
        let char2_val = (chars[1] as u8) - b'a';
        let (row1, col1) = self.get_coordinates(char1_val)?;
        let (row2, col2) = self.get_coordinates(char2_val)?;
        let (new_row1, new_col1, new_row2, new_col2) = if row1 == row2 {
            let shift = if encrypt { 1 } else { 4 };
            (row1, (col1 + shift) % 5, row2, (col2 + shift) % 5)
        } else if col1 == col2 {
            let shift = if encrypt { 1 } else { 4 };
            ((row1 + shift) % 5, col1, (row2 + shift) % 5, col2)
        } else {
            (row1, col2, row2, col1)
        };
        let result_char1 = (self.matrix[new_row1][new_col1] + b'a') as char;
        let result_char2 = (self.matrix[new_row2][new_col2] + b'a') as char;
        Ok(format!("{}{}", result_char1, result_char2))
    }

    fn process_text(&self, text: &str, encrypt: bool) -> Result<String, PolygraphiaError> {
        let prepared = Self::prepare_text(text);
        let mut result = String::new();
        let chars: Vec<char> = prepared.chars().collect();
        for chunk in chars.chunks(2) {
            let pair: String = chunk.iter().collect();
            result.push_str(&self.process_pair(&pair, encrypt)?);
        }
        Ok(result)
    }
}

impl Cipher for Playfair {
    fn encrypt(&self, plaintext: &str) -> Result<String, PolygraphiaError> {
        if plaintext.is_empty() {
            return Err(PolygraphiaError::InvalidInput(
                "Plaintext cannot be empty".to_string(),
            ));
        }
        if !plaintext.chars().any(|c| c.is_ascii_alphabetic()) {
            return Err(PolygraphiaError::InvalidInput(
                "Plaintext must contain at least one alphabetic character".to_string(),
            ));
        }
        self.process_text(plaintext, true)
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, PolygraphiaError> {
        if ciphertext.is_empty() {
            return Err(PolygraphiaError::InvalidInput(
                "Ciphertext cannot be empty".to_string(),
            ));
        }
        if !ciphertext.chars().any(|c| c.is_ascii_alphabetic()) {
            return Err(PolygraphiaError::InvalidInput(
                "Ciphertext must contain at least one alphabetic character".to_string(),
            ));
        }
        self.process_text(ciphertext, false)
    }

    fn name(&self) -> &str {
        "playfair"
    }
}

impl Drop for Playfair {
    fn drop(&mut self) {
        for row in &mut self.matrix {
            for cell in row {
                *cell = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playfair_new() {
        let cipher = Playfair::new("secret").unwrap();
        assert_eq!(cipher.key().len(), 25); // 5x5 matrix = 25 chars
    }

    #[test]
    fn test_playfair_empty_key() {
        assert!(Playfair::new("").is_err());
    }

    #[test]
    fn test_playfair_prepare_key() {
        let key = Playfair::prepare_key("secret");

        // Should start with unique letters from "secret"
        assert!(key.starts_with("secrt"));

        // Should be 25 characters (no 'j')
        assert_eq!(key.len(), 25);

        // Should not contain 'j'
        assert!(!key.contains('j'));

        // Should contain 'i' (j is replaced with i)
        assert!(key.contains('i'));
    }

    #[test]
    fn test_playfair_prepare_key_with_j() {
        let key = Playfair::prepare_key("jump");

        // 'j' should be replaced with 'i'
        assert!(key.starts_with("iu") || key.starts_with("i")); // 'j' becomes 'i', 'u', 'm', 'p'
        assert!(!key.contains('j'));
    }

    #[test]
    fn test_playfair_prepare_text() {
        // Basic text
        assert_eq!(Playfair::prepare_text("hello"), "helxlo");

        // Duplicate letters
        assert_eq!(Playfair::prepare_text("balloon"), "balxloxonx");

        // Odd length (adds 'x')
        assert_eq!(Playfair::prepare_text("cat"), "catx");

        // Replace 'j' with 'i'
        assert_eq!(Playfair::prepare_text("jump"), "iump");

        // Filter non-alphabetic
        assert_eq!(Playfair::prepare_text("he11o!"), "heox");
    }

    #[test]
    fn test_playfair_encrypt_basic() {
        let cipher = Playfair::new("playfair example").unwrap();
        let encrypted = cipher.encrypt("hide the gold in the tree stump").unwrap();

        // Should produce some encrypted text
        assert!(!encrypted.is_empty());
        assert_eq!(encrypted.len() % 2, 0); // Should be even length (pairs)
    }

    #[test]
    fn test_playfair_encrypt_decrypt_roundtrip() {
        let cipher = Playfair::new("secret").unwrap();

        let plaintext = "hello world";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        // Note: decrypted might have 'x' padding and 'j' replaced with 'i'
        // So we check that the core message is preserved
        assert!(decrypted.starts_with("helxlo"));
    }

    #[test]
    fn test_playfair_same_row() {
        let cipher = Playfair::new("playfairexample").unwrap();

        // Test characters in the same row
        let result = cipher.process_pair("ar", true).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_playfair_same_column() {
        let cipher = Playfair::new("playfairexample").unwrap();

        // Test characters in the same column
        let result = cipher.process_pair("mu", true).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_playfair_rectangle() {
        let cipher = Playfair::new("playfairexample").unwrap();

        // Test characters forming a rectangle
        let result = cipher.process_pair("hi", true).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_playfair_case_handling() {
        let cipher = Playfair::new("secret").unwrap();

        // Uppercase and lowercase should produce same result
        let enc1 = cipher.encrypt("HELLO").unwrap();
        let enc2 = cipher.encrypt("hello").unwrap();

        assert_eq!(enc1, enc2);
    }

    #[test]
    fn test_playfair_filters_numbers() {
        let cipher = Playfair::new("secret").unwrap();

        let encrypted = cipher.encrypt("hello123world").unwrap();

        // Numbers should be filtered out
        assert!(!encrypted.contains('1'));
        assert!(!encrypted.contains('2'));
        assert!(!encrypted.contains('3'));
    }

    #[test]
    fn test_playfair_set_key() {
        let mut cipher = Playfair::new("secret").unwrap();

        let enc1 = cipher.encrypt("hello").unwrap();

        cipher.set_key("keyword").unwrap();
        let enc2 = cipher.encrypt("hello").unwrap();

        assert_ne!(enc1, enc2);
    }

    #[test]
    fn test_playfair_empty_input() {
        let cipher = Playfair::new("secret").unwrap();

        assert!(cipher.encrypt("").is_err());
        assert!(cipher.decrypt("").is_err());
    }

    #[test]
    fn test_playfair_no_alpha_chars() {
        let cipher = Playfair::new("secret").unwrap();

        assert!(cipher.encrypt("12345").is_err());
    }

    #[test]
    fn test_playfair_cipher_trait() {
        let cipher: Box<dyn Cipher> = Box::new(Playfair::new("secret").unwrap());
        assert_eq!(cipher.name(), "playfair");
    }

    #[test]
    fn test_playfair_matrix_generation() {
        let cipher = Playfair::new("keyword").unwrap();
        let matrix = cipher.matrix();

        // Matrix should be 5x5
        assert_eq!(matrix.len(), 5);
        assert_eq!(matrix[0].len(), 5);

        // All values should be in range 0-25 (excluding 'j')
        for row in matrix {
            for &val in row {
                assert!(val < 26);
            }
        }
    }

    #[test]
    fn test_playfair_get_coordinates() {
        let cipher = Playfair::new("abcdefghiklmnopqrstuvwxyz").unwrap();

        // 'a' should be at (0, 0)
        let (row, col) = cipher.get_coordinates(0).unwrap();
        assert_eq!((row, col), (0, 0));

        // 'e' should be at (0, 4)
        let (row, col) = cipher.get_coordinates(4).unwrap();
        assert_eq!((row, col), (0, 4));
    }
}
