use crate::PolygraphiaError;

pub fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn are_coprime(a: u8, b: u8) -> bool {
    gcd(a, b) == 1
}

pub fn mod_inverse(a: u8, m: u8) -> Result<u8, PolygraphiaError> {
    if !are_coprime(a, m) {
        return Err(PolygraphiaError::InvalidInput(format!(
            "Modular inverse does not exist for {a} mod {m} (not coprime)"
        )));
    }
    for i in 1..m {
        if (a as u16 * i as u16).rem_euclid(m as u16) == 1 {
            return Ok(i);
        }
    }
    Err(PolygraphiaError::InvalidInput(format!(
        "Failed to compute modular inverse for {a} mod {m}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 26), 1);
        assert_eq!(gcd(26, 13), 13);
        assert_eq!(gcd(5, 26), 1);
        assert_eq!(gcd(2, 26), 2);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 26).unwrap(), 9);
        assert_eq!(mod_inverse(5, 26).unwrap(), 21);
        assert_eq!(mod_inverse(7, 26).unwrap(), 15);
        assert_eq!(mod_inverse(9, 26).unwrap(), 3);

        // Verify the inverse property
        for a in [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25] {
            let inv = mod_inverse(a, 26).unwrap() as u16;
            assert_eq!((a as u16 * inv) % 26, 1);
        }
    }

    #[test]
    fn test_mod_inverse_no_inverse() {
        // Even numbers (except 1) don't have inverse mod 26
        assert!(mod_inverse(2, 26).is_err());
        assert!(mod_inverse(4, 26).is_err());
        assert!(mod_inverse(13, 26).is_err());
    }

    #[test]
    fn test_are_coprime() {
        assert!(are_coprime(5, 26));
        assert!(are_coprime(3, 26));
        assert!(!are_coprime(2, 26));
        assert!(!are_coprime(13, 26));
    }
}
