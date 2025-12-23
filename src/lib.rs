mod classic;
mod traits;
mod utils;
mod error;

#[cfg(test)]
mod tests {
    use crate::classic::Caesar;
    use crate::traits::Cipher;

    #[test]
    fn test_caesar_new_with_modulo() {
        let cipher = Caesar::new(29).unwrap();
        assert_eq!(cipher.shift(), 3); // 29 % 26 = 3
    }

    #[test]
    fn test_caesar_invalid_shift() {
        assert!(Caesar::new(0).is_err());
        assert!(Caesar::new(-5).is_err());
    }

    #[test]
    fn test_caesar_encrypt() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");
        assert_eq!(cipher.encrypt("HELLO").unwrap(), "khoor");
        assert_eq!(cipher.encrypt("Hello World!").unwrap(), "khoorzruog");
    }

    #[test]
    fn test_caesar_decrypt() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.decrypt("khoor").unwrap(), "hello");
        assert_eq!(cipher.decrypt("KHOOR").unwrap(), "hello");
    }

    #[test]
    fn test_caesar_encrypt_decrypt_roundtrip() {
        let cipher = Caesar::new(7).unwrap();
        let plaintext = "The Quick Brown Fox Jumps Over The Lazy Dog";
        let encrypted = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        // Compare without spaces and case
        let original_clean: String = plaintext.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        assert_eq!(decrypted, original_clean);
    }

    #[test]
    fn test_caesar_wrap_around() {
        let cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("xyz").unwrap(), "abc");
        assert_eq!(cipher.decrypt("abc").unwrap(), "xyz");
    }

    #[test]
    fn test_caesar_set_shift() {
        let mut cipher = Caesar::new(3).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");

        cipher.set_shift(5).unwrap();
        assert_eq!(cipher.encrypt("hello").unwrap(), "mjqqt");
    }

    #[test]
    fn test_caesar_empty_input() {
        let cipher = Caesar::new(3).unwrap();
        assert!(cipher.encrypt("").is_err());
        assert!(cipher.decrypt("").is_err());
    }

    #[test]
    fn test_caesar_no_alpha_characters() {
        let cipher = Caesar::new(3).unwrap();
        assert!(cipher.encrypt("123!@#").is_err());
    }

    #[test]
    fn test_cipher_trait() {
        let cipher: Box<dyn Cipher> = Box::new(Caesar::new(3).unwrap());
        assert_eq!(cipher.name(), "Caesar");
        assert_eq!(cipher.encrypt("hello").unwrap(), "khoor");
    }
}
