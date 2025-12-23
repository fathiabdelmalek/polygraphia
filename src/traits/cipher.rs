use crate::error::PolygraphiaError;

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> Result<String, PolygraphiaError>;
    fn decrypt(&self, ciphertext: &str) -> Result<String, PolygraphiaError>;
    fn name(&self) -> &str;
}
