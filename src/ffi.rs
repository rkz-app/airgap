// src/ffi - C FFI interface for iOS and Android

use std::os::raw::c_int;
use std::slice;
use std::ptr;

// Only import when not generating bindings
#[cfg(not(cbindgen))]
use crate::{Decoder, Encoder, TransportError};

// ============================================================================
// ERROR CODES
// ============================================================================

pub const AIRGAP_OK: c_int = 0;
pub const AIRGAP_UNKNOWN_ERR: c_int = -1;
pub const AIRGAP_ERR_NULL_POINTER: c_int = -2;
pub const AIRGAP_ERR_INVALID_MAGIC: c_int = -3;
pub const AIRGAP_ERR_UNSUPPORTED_VERSION: c_int = -4;
pub const AIRGAP_ERR_CRC_MISMATCH: c_int = -5;
pub const AIRGAP_ERR_SESSION_MISMATCH: c_int = -6;
pub const AIRGAP_ERR_METADATA_MISMATCH: c_int = -7;
pub const AIRGAP_ERR_CHUNK_OUT_OF_BOUNDS: c_int = -8;
pub const AIRGAP_ERR_TOO_MANY_CHUNKS: c_int = -9;
pub const AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE: c_int = -10;
pub const AIRGAP_ERR_CHUNK_SIZE_TOO_SMALL: c_int = -11;
pub const AIRGAP_ERR_MISSING_CHUNK: c_int = -12;
pub const AIRGAP_ERR_ENCODING: c_int = -13;

pub enum AirgapEncoder {}

pub enum AirgapDecoder {}

#[repr(C)]
pub struct ByteArray {
    /// Pointer to byte data (may be NULL if empty)
    pub data: *mut u8,
    /// Length of data in bytes
    pub len: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_encoder_new(
    data: *const u8,
    data_len: usize,
    chunk_size: usize,
) -> *mut AirgapEncoder {
    if data.is_null() {
        return ptr::null_mut();
    }

    let data_slice = unsafe { slice::from_raw_parts(data, data_len) };

    match Encoder::new(data_slice, chunk_size) {
        Ok(encoder) => Box::into_raw(Box::new(encoder)) as *mut AirgapEncoder,
        Err(_) => ptr::null_mut(),
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
    result: *mut ByteArray,
) -> isize {
    if encoder.is_null() {
        return AIRGAP_ERR_NULL_POINTER as isize;
    }

    if result.is_null() {
        return AIRGAP_ERR_NULL_POINTER as isize;
    }

    let png = match (*(encoder as *const Encoder)).generate_png_bytes_for_item(index) {
        Ok(p) => p,
        Err(e) => {
            return error_to_code(e) as isize;
        }
    };

    *result = ByteArray::from_vec(png);
    AIRGAP_OK as isize
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
    (*(decoder as *const Decoder)).progress().1
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_get_received(decoder: *const AirgapDecoder) -> usize {
    if decoder.is_null() {
        return 0;
    }
    (*(decoder as *const Decoder)).progress().0
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_process_qr(
    decoder: *mut AirgapDecoder,
    qr_string: *const std::os::raw::c_char,
) -> isize {
    if decoder.is_null() {
        return AIRGAP_ERR_NULL_POINTER as isize;
    }

    if qr_string.is_null() {
        return AIRGAP_ERR_NULL_POINTER as isize;
    }

    let c_str = std::ffi::CStr::from_ptr(qr_string);
    let qr_data = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return AIRGAP_ERR_ENCODING as isize,
    };

    match (*(decoder as *mut Decoder)).process_qr_string(qr_data) {
        Ok(_) => AIRGAP_OK as isize,
        Err(err) => error_to_code(err) as isize,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn airgap_decoder_get_data(
    decoder: *const AirgapDecoder,
    result: *mut ByteArray,
) -> isize {
    if decoder.is_null() {
        return AIRGAP_ERR_NULL_POINTER as isize;
    }

    if result.is_null() {
        return AIRGAP_ERR_NULL_POINTER as isize;
    }

    match (*(decoder as *const Decoder)).get_data() {
        Ok(vec) => {
            *result = ByteArray::from_vec(vec);
            AIRGAP_OK as isize
        }
        Err(err) => error_to_code(err) as isize,
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

#[cfg(not(cbindgen))]
fn error_to_code(err: TransportError) -> c_int {
    match err {
        TransportError::UnknownError => AIRGAP_UNKNOWN_ERR,
        TransportError::InvalidMagic => AIRGAP_ERR_INVALID_MAGIC,
        TransportError::UnsupportedVersion(_) => AIRGAP_ERR_UNSUPPORTED_VERSION,
        TransportError::CrcMismatch => AIRGAP_ERR_CRC_MISMATCH,
        TransportError::MetadataMismatch => AIRGAP_ERR_METADATA_MISMATCH,
        TransportError::SessionMismatch => AIRGAP_ERR_SESSION_MISMATCH,
        TransportError::ChunkOutOfBounds(_) => AIRGAP_ERR_CHUNK_OUT_OF_BOUNDS,
        TransportError::TooManyChunks(_) => AIRGAP_ERR_TOO_MANY_CHUNKS,
        TransportError::ChunkSizeTooLarge(_, _) => AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE,
        TransportError::ChunkSizeTooSmall(_, _) => AIRGAP_ERR_CHUNK_SIZE_TOO_SMALL,
        TransportError::MissingChunk(_) => AIRGAP_ERR_MISSING_CHUNK,
        TransportError::EncodingError(_) => AIRGAP_ERR_ENCODING,
    }
}