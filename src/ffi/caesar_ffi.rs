use std::os::raw::c_char;
use crate::classic::Caesar;
use crate::traits::Cipher;
use crate::ffi::types::{CResult, c_str_to_rust};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn caesar_encrypt(shift: u8, plaintext: *const c_char) -> CResult {
    let plaintext = match unsafe { c_str_to_rust(plaintext) } {
        Ok(s) => s,
        Err(e) => return CResult::error(e),
    };

    let cipher = match Caesar::new(shift) {
        Ok(c) => c,
        Err(e) => return CResult::error(e.to_string()),
    };

    match cipher.encrypt(&plaintext) {
        Ok(ciphertext) => CResult::success(ciphertext),
        Err(e) => CResult::error(e.to_string()),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn caesar_decrypt(shift: u8, ciphertext: *const c_char) -> CResult {
    let ciphertext = match unsafe { c_str_to_rust(ciphertext) } {
        Ok(s) => s,
        Err(e) => return CResult::error(e),
    };

    let cipher = match Caesar::new(shift) {
        Ok(c) => c,
        Err(e) => return CResult::error(e.to_string()),
    };

    match cipher.decrypt(&ciphertext) {
        Ok(plaintext) => CResult::success(plaintext),
        Err(e) => CResult::error(e.to_string()),
    }
}
