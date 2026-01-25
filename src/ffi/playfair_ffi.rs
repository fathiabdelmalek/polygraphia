use std::os::raw::c_char;
use crate::classic::Playfair;
use crate::traits::Cipher;
use crate::ffi::types::{CResult, c_str_to_rust};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn playfair_encrypt(key: *const c_char, plaintext: *const c_char) -> CResult {
    let key = match unsafe { c_str_to_rust(key) } {
        Ok(s) => s,
        Err(e) => return CResult::error(e),
    };

    let plaintext = match unsafe { c_str_to_rust(plaintext) } {
        Ok(s) => s,
        Err(e) => return CResult::error(e),
    };

    let cipher = match Playfair::new(&key) {
        Ok(c) => c,
        Err(e) => return CResult::error(e.to_string()),
    };

    match cipher.encrypt(&plaintext) {
        Ok(ciphertext) => CResult::success(ciphertext),
        Err(e) => CResult::error(e.to_string()),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn playfair_decrypt(key: *const c_char, ciphertext: *const c_char) -> CResult {
    let key = match unsafe { c_str_to_rust(key) } {
        Ok(s) => s,
        Err(e) => return CResult::error(e),
    };

    let ciphertext = match unsafe { c_str_to_rust(ciphertext) } {
        Ok(s) => s,
        Err(e) => return CResult::error(e),
    };

    let cipher = match Playfair::new(&key) {
        Ok(c) => c,
        Err(e) => return CResult::error(e.to_string()),
    };

    match cipher.decrypt(&ciphertext) {
        Ok(plaintext) => CResult::success(plaintext),
        Err(e) => CResult::error(e.to_string()),
    }
}
