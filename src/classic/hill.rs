use crate::error::PolygraphiaError;
use crate::traits::Cipher;
use crate::utils::{Matrix, TextMode};

#[derive(Debug, Clone)]
pub struct Hill {
    key: Matrix,
    inv_key: Matrix,
    key_size: usize,
    mode: TextMode,
}

impl Hill {
    pub fn new(key: &str) -> Result<Self, PolygraphiaError> {
        Self::with_mode(key, TextMode::default())
    }

    pub fn key(&self) -> &Matrix {
        &self.key
    }

    pub fn inv_key(&self) -> &Matrix {
        &self.inv_key
    }

    pub fn key_size(&self) -> usize {
        self.key_size
    }

    pub fn mode(&self) -> TextMode {
        self.mode
    }

    pub fn set_key(&mut self, key: &str) -> Result<(), PolygraphiaError> {
        let (key_matrix, inv_key_matrix, key_size) = Self::prepare_key(key)?;
        self.key = key_matrix;
        self.inv_key = inv_key_matrix;
        self.key_size = key_size;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: TextMode) {
        self.mode = mode;
    }

    pub fn with_mode(key: &str, mode: TextMode) -> Result<Self, PolygraphiaError> {
        let (key_matrix, inv_key_matrix, key_size) = Self::prepare_key(key)?;
        Ok(Hill {
            key: key_matrix,
            inv_key: inv_key_matrix,
            key_size,
            mode,
        })
    }

    fn prepare_key(key: &str) -> Result<(Matrix, Matrix, usize), PolygraphiaError> {
        let key_clean: String = key
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let key_size = (key_clean.len() as f64).sqrt() as usize;
        if key_size * key_size != key_clean.len() {
            return Err(PolygraphiaError::InvalidKey(format!(
                "Key length {} must be a perfect square (4, 9, 16, 25, ...)",
                key_clean.len()
            )));
        }
        let matrix_data: Vec<i32> = key_clean.chars().map(|c| (c as u8 - b'a') as i32).collect();
        let key_matrix = Matrix::new(key_size, matrix_data)?;
        let inv_key_matrix = Self::validate_key(&key_matrix)?;
        Ok((key_matrix, inv_key_matrix, key_size))
    }

    fn validate_key(matrix: &Matrix) -> Result<Matrix, PolygraphiaError> {
        matrix.mod_inverse(26)
    }

    fn prepare_text(&self, text: &str) -> String {
        let mut clean: String = text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let remainder = clean.len() % self.key_size;
        if remainder != 0 {
            let padding_needed = self.key_size - remainder;
            clean.push_str(&"x".repeat(padding_needed));
        }
        clean
    }

    fn text_to_vector(text: &str) -> Vec<i32> {
        text.chars()
            .map(|c| (c.to_ascii_lowercase() as u8 - b'a') as i32)
            .collect()
    }

    fn vector_to_text(vector: &[i32]) -> String {
        vector
            .iter()
            .map(|&x| {
                let val = x.rem_euclid(26) as u8;
                (b'a' + val) as char
            })
            .collect()
    }

    fn process_text(&self, text: &str, encrypt: bool) -> String {
        let prepared = self.prepare_text(text);
        let vector = Self::text_to_vector(&prepared);
        let matrix = if encrypt { &self.key } else { &self.inv_key };
        let mut result = Vec::new();
        for chunk in vector.chunks(self.key_size) {
            let encrypted_chunk = matrix.multiply_vector(chunk);
            result.extend(encrypted_chunk.iter().map(|&x| x.rem_euclid(26)));
        }
        Self::vector_to_text(&result)
    }
}

impl Cipher for Hill {
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
        Ok(self.process_text(plaintext, true))
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
        Ok(self.process_text(ciphertext, false))
    }

    fn name(&self) -> &str {
        "hill"
    }
}

impl Drop for Hill {
    fn drop(&mut self) {
        // the matrix will clean automatically when it goes out of scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hill_new_2x2() {
        let cipher = Hill::new("hill").unwrap();
        assert_eq!(cipher.key_size(), 2);
    }

    #[test]
    fn test_hill_new_3x3() {
        let cipher = Hill::new("gybnqkurp").unwrap();
        assert_eq!(cipher.key_size(), 3);
    }

    #[test]
    fn test_hill_invalid_key_length() {
        // Length 5 is not a perfect square
        assert!(Hill::new("hello").is_err());

        // Length 7 is not a perfect square
        assert!(Hill::new("invalid").is_err());
    }

    #[test]
    fn test_hill_non_invertible_matrix() {
        // This matrix is not invertible mod 26
        let result = Hill::new("aaaa"); // All zeros -> det = 0
        assert!(result.is_err());
    }

    #[test]
    fn test_hill_encrypt_decrypt_2x2() {
        let cipher = Hill::new("hill").unwrap();

        let plaintext = "help";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_hill_encrypt_decrypt_3x3() {
        let cipher = Hill::new("gybnqkurp").unwrap();

        let plaintext = "act";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_hill_padding() {
        let cipher = Hill::new("hill").unwrap(); // 2x2 matrix

        // "cat" has 3 chars, needs 1 padding 'x' to make 4
        let encrypted = cipher.encrypt("cat").unwrap();
        assert_eq!(encrypted.len(), 4);

        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, "catx");
    }

    #[test]
    fn test_hill_filters_non_alpha() {
        let cipher = Hill::new("hill").unwrap();

        let plaintext = "he11o!";
        let encrypted = cipher.encrypt(plaintext).unwrap();

        // Should only process "heo" -> "heox" (with padding)
        assert_eq!(encrypted.len(), 4);
    }

    #[test]
    fn test_hill_case_handling() {
        let cipher = Hill::new("hill").unwrap();

        // Uppercase should be converted to lowercase
        let enc1 = cipher.encrypt("HELP").unwrap();
        let enc2 = cipher.encrypt("help").unwrap();

        assert_eq!(enc1, enc2);
    }

    #[test]
    fn test_hill_longer_text() {
        let cipher = Hill::new("hill").unwrap();

        let plaintext = "thequickbrownfox";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_hill_empty_input() {
        let cipher = Hill::new("hill").unwrap();

        assert!(cipher.encrypt("").is_err());
        assert!(cipher.decrypt("").is_err());
    }

    #[test]
    fn test_hill_no_alpha_chars() {
        let cipher = Hill::new("hill").unwrap();

        assert!(cipher.encrypt("123!@#").is_err());
    }

    #[test]
    fn test_hill_set_key() {
        let mut cipher = Hill::new("hill").unwrap();

        let enc1 = cipher.encrypt("help").unwrap();

        cipher.set_key("ddcf").unwrap();
        let enc2 = cipher.encrypt("help").unwrap();

        assert_ne!(enc1, enc2);
    }

    #[test]
    fn test_hill_cipher_trait() {
        let cipher: Box<dyn Cipher> = Box::new(Hill::new("hill").unwrap());
        assert_eq!(cipher.name(), "hill");
    }

    #[test]
    fn test_matrix_determinant_2x2() {
        let matrix = Matrix::new(2, vec![7, 8, 11, 11]).unwrap();
        let det = matrix.determinant();
        assert_eq!(det, 7 * 11 - 8 * 11);
    }

    #[test]
    fn test_matrix_determinant_3x3() {
        let matrix = Matrix::new(3, vec![6, 24, 1, 13, 16, 10, 20, 17, 15]).unwrap();
        let det = matrix.determinant();
        // Verify it's invertible mod 26
        assert_ne!(det % 26, 0);
    }

    #[test]
    fn test_matrix_mod_inverse() {
        let matrix = Matrix::new(2, vec![7, 8, 11, 11]).unwrap();
        let inv = matrix.mod_inverse(26).unwrap();

        // Verify that matrix * inv ≡ I (mod 26)
        // This is a basic sanity check
        assert_eq!(inv.size(), 2);
    }
}
