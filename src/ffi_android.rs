// src/ffi_android - JNI interface for Android

use jni::JNIEnv;
use jni::objects::{JClass, JByteArray, JObject, JString};
use jni::sys::{jlong, jint, jboolean, jbyteArray};
use crate::{Decoder, Encoder};

// ============================================================================
// ENCODER JNI FUNCTIONS
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeNew<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
    chunk_size: jint,
) -> jlong {
    let data_bytes: Vec<u8> = match env.convert_byte_array(&data) {
        Ok(bytes) => bytes,
        Err(_) => return 0,
    };

    match Encoder::new(&data_bytes, chunk_size as usize) {
        Ok(encoder) => Box::into_raw(Box::new(encoder)) as jlong,
        Err(_) => 0,
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
pub extern "system" fn Java_app_rkz_airgap_AirgapEncoder_nativeGeneratePng<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    index: jint,
) -> jbyteArray {
    if handle == 0 {
        return JObject::null().into_raw();
    }

    let encoder = unsafe { &*(handle as *const Encoder) };

    let png_bytes = match encoder.generate_png_bytes_for_item(index as usize) {
        Ok(bytes) => bytes,
        Err(_) => return JObject::null().into_raw(),
    };

    match env.byte_array_from_slice(&png_bytes) {
        Ok(arr) => {
            let arr: JByteArray<'local> = arr;
            arr.into_raw()
        }
        Err(_) => JObject::null().into_raw(),
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
    decoder.progress().1 as jint
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
    decoder.progress().0 as jint
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeProcessQr(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    qr_string: JObject,
) -> jint {
    if handle == 0 {
        return -1;
    }

    let jstring: jni::objects::JString = qr_string.into();
    let qr_str = match env.get_string(&jstring) {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let qr_data: String = qr_str.into();
    let decoder = unsafe { &mut *(handle as *mut Decoder) };

    match decoder.process_qr_string(&qr_data) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_app_rkz_airgap_AirgapDecoder_nativeGetData<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
) -> jbyteArray {
    if handle == 0 {
        return JObject::null().into_raw();
    }

    let decoder = unsafe { &*(handle as *const Decoder) };

    let data = match decoder.get_data() {
        Ok(d) => d,
        Err(_) => return JObject::null().into_raw(),
    };

    match env.byte_array_from_slice(&data) {
        Ok(arr) => {
            let arr: JByteArray<'local> = arr;
            arr.into_raw()
        }
        Err(_) => JObject::null().into_raw(),
    }
}