// src/ffi_android - JNI interface for Android

use jni::JNIEnv;
use jni::objects::{JClass, JByteArray, JObject};
use jni::sys::{jlong, jint, jboolean, jbyteArray};
use crate::{Decoder, Encoder, QrConfig};
use crate::error::AirgapError;

// Helper function to throw AirgapException
fn throw_exception(env: &mut JNIEnv, error: &AirgapError) {
    let _ = env.throw_new("app/rkz/airgap/AirgapException", error.to_string());
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
    qr_size: jint,
) -> jlong {
    let data_bytes: Vec<u8> = match env.convert_byte_array(&data) {
        Ok(bytes) => bytes,
        Err(_) => {
            let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to convert byte array");
            return 0;
        }
    };

    match Encoder::with_config(&data_bytes, chunk_size as usize, QrConfig::with_size(qr_size as u32)) {
        Ok(encoder) => Box::into_raw(Box::new(encoder)) as jlong,
        Err(err) => {
            throw_exception(&mut env, &err);
            0
        }
    }
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

    match encoder.get_qr_string(index as usize) {
        Ok(qr_string) => match env.new_string(&qr_string) {
            Ok(s) => s.into(),
            Err(_) => {
                let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create Java string");
                JObject::null()
            }
        },
        Err(err) => {
            throw_exception(&mut env, &err);
            JObject::null()
        }
    }
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

    match encoder.generate_png_bytes_for_item(index as usize) {
        Ok(png_bytes) => match env.byte_array_from_slice(&png_bytes) {
            Ok(arr) => arr.into_raw(),
            Err(_) => {
                let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create Java byte array");
                JObject::null().into_raw()
            }
        },
        Err(err) => {
            throw_exception(&mut env, &err);
            JObject::null().into_raw()
        }
    }
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

    match decoder.process_qr_string(&qr_data) {
        Ok(chunk) => {
            // Create QRResult Java object
            let qr_result_class = match env.find_class("app/rkz/airgap/QRResult") {
                Ok(cls) => cls,
                Err(_) => {
                    let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to find QRResult class");
                    return JObject::null();
                }
            };

            match env.new_object(
                qr_result_class,
                "(II)V",
                &[
                    jni::objects::JValue::Int(chunk.chunk_index as jint),
                    jni::objects::JValue::Int(chunk.total_chunks as jint),
                ],
            ) {
                Ok(obj) => obj,
                Err(_) => {
                    let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create QRResult object");
                    JObject::null()
                }
            }
        }
        Err(err) => {
            throw_exception(&mut env, &err);
            JObject::null()
        }
    }
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

    match decoder.get_data() {
        Ok(data) => match env.byte_array_from_slice(&data) {
            Ok(arr) => arr.into_raw(),
            Err(_) => {
                let _ = env.throw_new("app/rkz/airgap/AirgapException", "Failed to create Java byte array");
                JObject::null().into_raw()
            }
        },
        Err(err) => {
            throw_exception(&mut env, &err);
            JObject::null().into_raw()
        }
    }
}