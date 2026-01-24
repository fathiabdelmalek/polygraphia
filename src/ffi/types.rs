use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// FFI-safe result type
#[repr(C)]
pub struct CResult {
    pub success: bool,
    pub data: *mut c_char,
    pub error: *mut c_char,
}

impl CResult {
    pub fn success(data: String) -> Self {
        CResult {
            success: true,
            data: CString::new(data).unwrap().into_raw(),
            error: std::ptr::null_mut(),
        }
    }

    pub fn error(error: String) -> Self {
        CResult {
            success: false,
            data: std::ptr::null_mut(),
            error: CString::new(error).unwrap().into_raw(),
        }
    }
}

/// Helper to convert C string to Rust string
pub unsafe fn c_str_to_rust(c_str: *const c_char) -> Result<String, String> {
    if c_str.is_null() {
        return Err("Null pointer".to_string());
    }

    CStr::from_ptr(c_str)
        .to_str()
        .map(|s| s.to_string())
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}
