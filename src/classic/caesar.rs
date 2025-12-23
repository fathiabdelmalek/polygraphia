use std::fmt::Debug;
use crate::error::PolygraphiaError;
use crate::traits::Cipher;

#[derive(Debug, Clone)]
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: i8) -> Result<Self, PolygraphiaError> {
        Self::validate_shift(shift)?;
        Ok(Caesar {
            shift: (shift % 26) as u8,
        })
    }

    pub fn shift(&self) -> u8 {
        self.shift
    }

    pub fn set_shift(&mut self, shift: i8) -> Result<(), PolygraphiaError> {
        Self::validate_shift(shift)?;
        self.shift = (shift % 26) as u8;
        Ok(())
    }

    fn validate_shift(shift: i8) -> Result<(), PolygraphiaError> {
        if shift <= 0 {
            return Err(PolygraphiaError::InvalidKey(format!("Shift must be positive, got: {}", shift)));
        }
        Ok(())
    }

    fn prepare_text(text: &str) -> Vec<u8> {
        text.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| {
                let c_lower = c.to_ascii_lowercase();
                (c_lower as u8) - b'a'
            })
            .collect()
    }

    fn process_text(&self, indices: Vec<u8>, encrypt: bool) -> String {
        let shift = if encrypt {
            self.shift as i8
        } else {
            -(self.shift as i8)
        };
        indices
            .iter()
            .map(|&idx| {
                let shifted = (idx as i8 + shift).rem_euclid(26) as u8;
                (b'a' + shifted as u8) as char
            })
            .collect()
    }
}

impl Cipher for Caesar {
    fn encrypt(&self, plaintext: &str) -> Result<String, PolygraphiaError> {
        if plaintext.is_empty () {
            return Err(PolygraphiaError::InvalidInput("Empty plaintext".to_string()));
        }
        let indices = Self::prepare_text(plaintext);
        if indices.is_empty() {
            return Err(PolygraphiaError::InvalidInput("plaintext must have at least one alphabetic character".to_string()));
        }
        Ok(self.process_text(indices, true))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, PolygraphiaError> {
        if ciphertext.is_empty () {
            return Err(PolygraphiaError::InvalidInput("Empty ciphertext".to_string()));
        }
        let indices = Self::prepare_text(ciphertext);
        if indices.is_empty() {
            return Err(PolygraphiaError::InvalidInput("ciphertext must have at least one alphabetic character".to_string()));
        }
        Ok(self.process_text(indices, false))
    }

    fn name(&self) -> &str {
        "Caesar"
    }
}

impl Drop for Caesar {
    fn drop(&mut self) {
        self.shift = 0;
    }
}
