use core::ffi::{c_void, c_char, c_int};
use core::ptr::null_mut;
use alloc::ffi::CString;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use crate::error::AirgapError;

#[repr(C)]
pub struct CResult {
    pub code: c_int,
    pub payload: *const c_void,
    pub error_message: *const c_char,
}

pub const AIRGAP_OK: c_int = 0;

#[unsafe(no_mangle)]
pub extern "C" fn result_error_message_free(result: CResult) {
    if !result.error_message.is_null() {
        unsafe {
            let _ = CString::from_raw(result.error_message as *mut _);
        }
    }
}

#[cfg(not(cbindgen))]
impl CResult {
    pub fn from_success<V>(payload: Box<V>) -> Self {
        CResult {
            code: AIRGAP_OK,
            payload: Box::into_raw(payload) as *mut c_void,
            error_message: null_mut()
        }
    }
    pub fn from_custom_error(error_message: String, code: i32) -> Self {
        CResult {
            code: code as c_int,
            payload: null_mut(),
            error_message: CString::new(error_message).unwrap().into_raw(), // Transfer ownership to caller
        }
    }

    pub fn from_error(error: AirgapError) -> Self {
        let code = error.to_code() as c_int;
        let message = CString::new(error.to_string())
            .unwrap_or_else(|_| CString::new("Unknown error").unwrap());

        CResult {
            code,
            payload: null_mut(),
            error_message: message.into_raw(), // Transfer ownership to caller
        }
    }
}
