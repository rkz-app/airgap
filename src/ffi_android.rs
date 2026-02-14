// src/ffi_android - JNI interface for Android

use jni::JNIEnv;
use jni::objects::{JClass, JByteArray, JObject, JString};
use jni::sys::{jlong, jint, jboolean, jbyteArray};
use crate::{Decoder, Encoder};
use crate::ffi::{ByteArray, QRResult};
use crate::ffi_result::{CResult, AIRGAP_OK};
use std::ffi::CStr;

// Helper function to throw AirgapException with error message from CResult
fn throw_airgap_exception(env: &mut JNIEnv, result: &CResult) {
    let message = if !result.error_message.is_null() {
        unsafe {
            CStr::from_ptr(result.error_message)
                .to_str()
                .unwrap_or("Unknown error")
        }
    } else {
        "Operation failed"
    };

    let _ = env.throw_new("app/rkz/airgap/AirgapException", message);
}

// ============================================================================
// ENCODER JNI FUNCTIONS
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeNew<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
    chunk_size: jint,
) -> jlong {
    let data_bytes: Vec<u8> = match env.convert_byte_array(&data) {
        Ok(bytes) => bytes,
        Err(_) => {
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to convert byte array");
            return 0;
        }
    };

    let result = match Encoder::new(&data_bytes, chunk_size as usize) {
        Ok(encoder) => CResult::from_success(Box::new(encoder)),
        Err(err) => CResult::from_error(err),
    };

    if result.code != AIRGAP_OK {
        throw_airgap_exception(&mut env, &result);
        unsafe { crate::ffi_result::result_error_message_free(result); }
        return 0;
    }

    let encoder_ptr = result.payload as jlong;
    unsafe { crate::ffi_result::result_error_message_free(result); }
    encoder_ptr
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeFree(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        unsafe {
            drop(Box::from_raw(handle as *mut Encoder));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeChunkCount(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return 0;
    }
    let encoder = unsafe { &*(handle as *const Encoder) };
    encoder.chunk_count() as jint
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeSessionId(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return 0;
    }
    let encoder = unsafe { &*(handle as *const Encoder) };
    encoder.session_id() as jint
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeGetQRString<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    index: jint,
) -> JObject<'local> {
    if handle == 0 {
        let _ = env.throw_new("app/rkz/airgap/AirgapException", "Encoder handle is null");
        return JObject::null();
    }

    let encoder = unsafe { &*(handle as *const Encoder) };

    let result = match encoder.get_qr_string(index as usize) {
        Ok(qr_string) => CResult::from_success(Box::new(qr_string)),
        Err(err) => CResult::from_error(err),
    };

    if result.code != AIRGAP_OK {
        throw_airgap_exception(&mut env, &result);
        unsafe { crate::ffi_result::result_error_message_free(result); }
        return JObject::null();
    }

    let qr_string = unsafe { &*(result.payload as *const String) };

    let java_string = match env.new_string(qr_string) {
        Ok(s) => s,
        Err(_) => {
            unsafe {
                drop(Box::from_raw(result.payload as *mut String));
                crate::ffi_result::result_error_message_free(result);
            }
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create Java string");
            return JObject::null();
        }
    };

    // Free resources
    unsafe {
        drop(Box::from_raw(result.payload as *mut String));
        crate::ffi_result::result_error_message_free(result);
    }

    java_string.into()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeGeneratePng<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    index: jint,
) -> jbyteArray {
    if handle == 0 {
        let _ = env.throw_new("app/rkz/airgap/AirgapException", "Encoder handle is null");
        return JObject::null().into_raw();
    }

    let encoder = unsafe { &*(handle as *const Encoder) };

    let result = match encoder.generate_png_bytes_for_item(index as usize) {
        Ok(bytes) => CResult::from_success(Box::new(ByteArray::from_vec(bytes))),
        Err(err) => CResult::from_error(err),
    };

    if result.code != AIRGAP_OK {
        throw_airgap_exception(&mut env, &result);
        unsafe { crate::ffi_result::result_error_message_free(result); }
        return JObject::null().into_raw();
    }

    let byte_array = unsafe { &*(result.payload as *const ByteArray) };
    let png_bytes = unsafe { std::slice::from_raw_parts(byte_array.data, byte_array.len) };

    let java_array = match env.byte_array_from_slice(png_bytes) {
        Ok(arr) => arr.into_raw(),
        Err(_) => {
            unsafe {
                crate::ffi::airgap_byte_array_free(*byte_array);
                std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<ByteArray>());
                crate::ffi_result::result_error_message_free(result);
            }
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create Java byte array");
            return JObject::null().into_raw();
        }
    };

    // Free resources
    unsafe {
        crate::ffi::airgap_byte_array_free(*byte_array);
        std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<ByteArray>());
        crate::ffi_result::result_error_message_free(result);
    }

    java_array
}

// ============================================================================
// DECODER JNI FUNCTIONS
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(Decoder::new())) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeFree(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        unsafe {
            drop(Box::from_raw(handle as *mut Decoder));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeIsComplete(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jboolean {
    if handle == 0 {
        return 0;
    }
    let decoder = unsafe { &*(handle as *const Decoder) };
    if decoder.is_complete() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeGetTotal(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return 0;
    }
    let decoder = unsafe { &*(handle as *const Decoder) };
    decoder.total_count() as jint
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeGetReceived(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return 0;
    }
    let decoder = unsafe { &*(handle as *const Decoder) };
    decoder.received_count() as jint
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeGetSessionId(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jint {
    if handle == 0 {
        return -1;
    }
    let decoder = unsafe { &*(handle as *const Decoder) };
    decoder.session_id().map(|id| id as jint).unwrap_or(-1)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeReset(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle == 0 {
        return;
    }
    let decoder = unsafe { &mut *(handle as *mut Decoder) };
    decoder.reset();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeProcessQr<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    qr_string: JObject<'local>,
) -> JObject<'local> {
    if handle == 0 {
        let _ = env.throw_new("app/rkz/airgap/AirgapException", "Decoder handle is null");
        return JObject::null();
    }

    let jstring: jni::objects::JString = qr_string.into();
    let qr_str = match env.get_string(&jstring) {
        Ok(s) => s,
        Err(_) => {
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to get string");
            return JObject::null();
        }
    };

    let qr_data: String = qr_str.into();
    let decoder = unsafe { &mut *(handle as *mut Decoder) };

    let result = match decoder.process_qr_string(&qr_data) {
        Ok(chunk) => CResult::from_success(Box::new(QRResult {
            chunk_number: chunk.chunk_index as usize,
            total_chunk_count: chunk.total_chunks as usize,
        })),
        Err(err) => CResult::from_error(err),
    };

    if result.code != AIRGAP_OK {
        throw_airgap_exception(&mut env, &result);
        unsafe { crate::ffi_result::result_error_message_free(result); }
        return JObject::null();
    }

    let qr_result = unsafe { &*(result.payload as *const QRResult) };

    // Create QRResult Java object
    let qr_result_class = match env.find_class("app/rkz/airgap/QRResult") {
        Ok(cls) => cls,
        Err(_) => {
            unsafe {
                std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<QRResult>());
                crate::ffi_result::result_error_message_free(result);
            }
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to find QRResult class");
            return JObject::null();
        }
    };

    let java_qr_result = match env.new_object(
        qr_result_class,
        "(II)V",
        &[
            jni::objects::JValue::Int(qr_result.chunk_number as jint),
            jni::objects::JValue::Int(qr_result.total_chunk_count as jint),
        ],
    ) {
        Ok(obj) => obj,
        Err(_) => {
            unsafe {
                std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<QRResult>());
                crate::ffi_result::result_error_message_free(result);
            }
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create QRResult object");
            return JObject::null();
        }
    };

    // Free resources
    unsafe {
        std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<QRResult>());
        crate::ffi_result::result_error_message_free(result);
    }

    java_qr_result
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeGetData<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
) -> jbyteArray {
    if handle == 0 {
        let _ = env.throw_new("app/rkz/airgap/AirgapException", "Decoder handle is null");
        return JObject::null().into_raw();
    }

    let decoder = unsafe { &*(handle as *const Decoder) };

    let result = match decoder.get_data() {
        Ok(data) => CResult::from_success(Box::new(ByteArray::from_vec(data))),
        Err(err) => CResult::from_error(err),
    };

    if result.code != AIRGAP_OK {
        throw_airgap_exception(&mut env, &result);
        unsafe { crate::ffi_result::result_error_message_free(result); }
        return JObject::null().into_raw();
    }

    let byte_array = unsafe { &*(result.payload as *const ByteArray) };
    let data_bytes = unsafe { std::slice::from_raw_parts(byte_array.data, byte_array.len) };

    let java_array = match env.byte_array_from_slice(data_bytes) {
        Ok(arr) => arr.into_raw(),
        Err(_) => {
            unsafe {
                crate::ffi::airgap_byte_array_free(*byte_array);
                std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<ByteArray>());
                crate::ffi_result::result_error_message_free(result);
            }
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create Java byte array");
            return JObject::null().into_raw();
        }
    };

    // Free resources
    unsafe {
        crate::ffi::airgap_byte_array_free(*byte_array);
        std::alloc::dealloc(result.payload as *mut u8, std::alloc::Layout::new::<ByteArray>());
        crate::ffi_result::result_error_message_free(result);
    }

    java_array
}