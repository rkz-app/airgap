// src/ffi - C FFI interface for iOS and Android

use std::os::raw::c_int;
use std::ptr;
use std::slice;

// Only import when not generating bindings
#[cfg(not(cbindgen))]
use crate::{Decoder, Encoder};
use crate::error::AirgapError;
use crate::ffi_result::{CResult, AIRGAP_OK};

pub enum AirgapEncoder {}

pub enum AirgapDecoder {}

#[repr(C)]
pub struct ByteArray {
    pub data: *mut u8,
    pub len: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_encoder_new(
    data: *const u8,
    data_len: usize,
    chunk_size: usize,
) -> CResult {
    if data.is_null() {
        return CResult::from_error(AirgapError::UnknownError)
    }

    let data_slice = unsafe { slice::from_raw_parts(data, data_len) };

    match Encoder::new(data_slice, chunk_size) {
        Ok(encoder) => CResult::from_success(Box::new(encoder)),
        Err(err) => CResult::from_error(err),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_encoder_free(encoder: *mut AirgapEncoder) {
    if !encoder.is_null() {
        drop(Box::from_raw(encoder as *mut Encoder));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_encoder_chunk_count(encoder: *const AirgapEncoder) -> usize {
    if encoder.is_null() {
        return 0;
    }
    (*(encoder as *const Encoder)).chunk_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_encoder_session_id(encoder: *const AirgapEncoder) -> u32 {
    if encoder.is_null() {
        return 0;
    }
    (*(encoder as *const Encoder)).session_id()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_encoder_generate_png(
    encoder: *const AirgapEncoder,
    index: usize,
) -> CResult {
    if encoder.is_null() {
        return CResult::from_custom_error("encoder null ptr".to_string(), -1);
    }

    let png = match (*(encoder as *const Encoder)).generate_png_bytes_for_item(index) {
        Ok(p) => p,
        Err(e) => {
            return CResult::from_error(e);
        }
    };
    CResult::from_success(Box::new(ByteArray::from_vec(png)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_new() -> *mut AirgapDecoder {
    Box::into_raw(Box::new(Decoder::new())) as *mut AirgapDecoder
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_free(decoder: *mut AirgapDecoder) {
    if !decoder.is_null() {
        drop(Box::from_raw(decoder as *mut Decoder));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_is_complete(decoder: *const AirgapDecoder) -> bool {
    if decoder.is_null() {
        return false;
    }
    (*(decoder as *const Decoder)).is_complete()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_get_total(decoder: *const AirgapDecoder) -> usize {
    if decoder.is_null() {
        return 0;
    }
    (*(decoder as *const Decoder)).total_count()
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_get_received(decoder: *const AirgapDecoder) -> usize {
    if decoder.is_null() {
        return 0;
    }
    (*(decoder as *const Decoder)).received_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_get_session_id(decoder: *const AirgapDecoder) -> isize {
    if decoder.is_null() {
        return 0;
    }
    match (*(decoder as *const Decoder)).session_id() {
        Some(session_id) => session_id as isize,
        None => -1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_reset(decoder: *const AirgapDecoder) -> c_int{
    if decoder.is_null() {
        return -1;
    }
    (*(decoder as *mut Decoder)).reset();
    AIRGAP_OK
}

#[repr(C)]
pub struct QRResult {
    pub chunk_number: usize,
    pub total_chunk_count: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_process_qr(
    decoder: *mut AirgapDecoder,
    qr_string: *const std::os::raw::c_char,
) -> CResult {
    if decoder.is_null() {
        return CResult::from_custom_error("decoder null ptr".to_string(), -1);
    }

    if qr_string.is_null() {
        return CResult::from_custom_error("qr_string null ptr".to_string(), -1);
    }

    let c_str = std::ffi::CStr::from_ptr(qr_string);
    let qr_data = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CResult::from_custom_error("c str conv".to_string(), -2),
    };

    match (*(decoder as *mut Decoder)).process_qr_string(qr_data) {
        Ok(chunk) => CResult::from_success(Box::new(QRResult{ chunk_number: chunk.chunk_index as usize, total_chunk_count: chunk.total_chunks as usize })),
        Err(err) => CResult::from_error(err),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_get_data(
    decoder: *const AirgapDecoder,
) -> CResult {
    if decoder.is_null() {
        return CResult::from_custom_error("decoder null ptr".to_string(), -1);
    }

    match (*(decoder as *const Decoder)).get_data() {
        Ok(vec) => {
            CResult::from_success(Box::new(ByteArray::from_vec(vec)))
        }
        Err(err) => CResult::from_error(err),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_byte_array_free(array: ByteArray) {
    if !array.is_null() {
        let _ = Vec::from_raw_parts(array.data, array.len, array.len);
    }
}

#[cfg(not(cbindgen))]
impl ByteArray {
    pub fn from_vec(mut vec: Vec<u8>) -> Self {
        let data = vec.as_mut_ptr();
        let len = vec.len();
        std::mem::forget(vec);
        Self { data, len }
    }

    pub fn empty() -> Self {
        Self {
            data: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn is_null(&self) -> bool {
        self.data.is_null()
    }
}
