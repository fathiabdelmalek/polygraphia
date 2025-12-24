use base64::{Engine as _, engine::general_purpose};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha512_256;

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_key(password: &str, salt: &[u8]) -> String {
    derive_key_with_iterations(password, salt, 100_000)
}

pub fn derive_key_with_iterations(password: &str, salt: &[u8], iterations: u32) -> String {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha512_256>(password.as_bytes(), salt, iterations, &mut key);
    general_purpose::URL_SAFE.encode(key)
}

pub fn derive_key_raw(password: &str, salt: &[u8], iterations: u32) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha512_256>(password.as_bytes(), salt, iterations, &mut key);
    key
}

pub fn verify_password(password: &str, salt: &[u8], expected_key: &str, iterations: u32) -> bool {
    let derived = derive_key_with_iterations(password, salt, iterations);
    constant_time_compare(derived.as_bytes(), expected_key.as_bytes())
}

fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_salt() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        // Should be 16 bytes
        assert_eq!(salt1.len(), 16);
        assert_eq!(salt2.len(), 16);

        // Should be different (probabilistically)
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_derive_key() {
        let password = "my_secure_password";
        let salt = generate_salt();

        let key = derive_key(password, &salt);

        // Should be base64 encoded (44 chars for 32 bytes)
        assert!(!key.is_empty());

        // Same password and salt should produce same key
        let key2 = derive_key(password, &salt);
        assert_eq!(key, key2);
    }

    #[test]
    fn test_derive_key_different_salts() {
        let password = "password123";
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        let key1 = derive_key(password, &salt1);
        let key2 = derive_key(password, &salt2);

        // Different salts should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_key_different_passwords() {
        let salt = generate_salt();

        let key1 = derive_key("password1", &salt);
        let key2 = derive_key("password2", &salt);

        // Different passwords should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_key_with_iterations() {
        let password = "test_password";
        let salt = generate_salt();

        let key1 = derive_key_with_iterations(password, &salt, 10_000);
        let key2 = derive_key_with_iterations(password, &salt, 10_000);

        // Same parameters should produce same key
        assert_eq!(key1, key2);

        // Different iterations should produce different keys
        let key3 = derive_key_with_iterations(password, &salt, 20_000);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_derive_key_raw() {
        let password = "raw_key_test";
        let salt = generate_salt();

        let raw_key = derive_key_raw(password, &salt, 100_000);

        // Should be 32 bytes
        assert_eq!(raw_key.len(), 32);

        // Should be consistent
        let raw_key2 = derive_key_raw(password, &salt, 100_000);
        assert_eq!(raw_key, raw_key2);
    }

    #[test]
    fn test_verify_password() {
        let password = "correct_password";
        let salt = generate_salt();
        let derived_key = derive_key(password, &salt);

        // Correct password should verify
        assert!(verify_password(password, &salt, &derived_key, 100_000));

        // Wrong password should not verify
        assert!(!verify_password(
            "wrong_password",
            &salt,
            &derived_key,
            100_000
        ));

        // Empty password should not verify
        assert!(!verify_password("", &salt, &derived_key, 100_000));
    }

    #[test]
    fn test_constant_time_compare() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";
        let d = b"helloworld";

        assert!(constant_time_compare(a, b));
        assert!(!constant_time_compare(a, c));
        assert!(!constant_time_compare(a, d)); // Different lengths
    }

    #[test]
    fn test_key_derivation_deterministic() {
        // Test that key derivation is deterministic
        let password = "deterministic_test";
        let salt = [42u8; 16]; // Fixed salt

        let key1 = derive_key(password, &salt);
        let key2 = derive_key(password, &salt);
        let key3 = derive_key(password, &salt);

        assert_eq!(key1, key2);
        assert_eq!(key2, key3);
    }

    #[test]
    fn test_base64_encoding() {
        let password = "test";
        let salt = [0u8; 16];
        let key = derive_key(password, &salt);

        // Should be valid base64
        assert!(general_purpose::URL_SAFE.decode(&key).is_ok());

        // Decoded should be 32 bytes
        let decoded = general_purpose::URL_SAFE.decode(&key).unwrap();
        assert_eq!(decoded.len(), 32);
    }
}
