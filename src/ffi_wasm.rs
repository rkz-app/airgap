// src/ffi_wasm - wasm-bindgen interface for JS/browser targets

use wasm_bindgen::prelude::*;
use crate::{Decoder, Encoder, QrConfig};

#[wasm_bindgen]
pub struct WasmQRResult {
    pub chunk_index: u16,
    pub total_chunks: u16,
}

// ============================================================================
// ENCODER
// ============================================================================

#[wasm_bindgen]
pub struct WasmEncoder {
    inner: Encoder,
}

#[wasm_bindgen]
impl WasmEncoder {
    /// Create an encoder from raw bytes.
    ///
    /// - `data`       – bytes to transmit
    /// - `chunk_size` – max payload bytes per QR chunk (16–1920, recommended ≤1100)
    /// - `qr_size`    – output PNG pixel dimensions (e.g. 400)
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8], chunk_size: usize, qr_size: u32) -> Result<WasmEncoder, JsError> {
        let inner = Encoder::with_config(data, chunk_size, QrConfig::with_size(qr_size))
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(Self { inner })
    }

    /// Number of QR code chunks needed to transmit the data.
    #[wasm_bindgen]
    pub fn chunk_count(&self) -> usize {
        self.inner.chunk_count()
    }

    /// Random session ID shared across all chunks of this transfer.
    #[wasm_bindgen]
    pub fn session_id(&self) -> u32 {
        self.inner.session_id()
    }

    /// Base45-encoded string for chunk at `index` — pass this to a JS QR renderer.
    #[wasm_bindgen]
    pub fn get_qr_string(&self, index: usize) -> Result<String, JsError> {
        self.inner
            .get_qr_string(index)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// PNG bytes for chunk at `index` — write into a canvas or an <img> src data URL.
    #[wasm_bindgen]
    pub fn generate_png(&self, index: usize) -> Result<Vec<u8>, JsError> {
        self.inner
            .generate_png_bytes_for_item(index)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ============================================================================
// DECODER
// ============================================================================

#[wasm_bindgen]
pub struct WasmDecoder {
    inner: Decoder,
}

#[wasm_bindgen]
impl WasmDecoder {
    /// Create a fresh decoder. Feed QR strings from a JS scanner via `process_qr`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmDecoder {
        Self { inner: Decoder::new() }
    }

    /// Feed a Base45 QR string (as decoded by jsQR / ZXing-js) into the decoder.
    /// Returns a `WasmQRResult` with `chunk_index` and `total_chunks` on success.
    #[wasm_bindgen]
    pub fn process_qr(&mut self, qr_string: &str) -> Result<WasmQRResult, JsError> {
        let chunk = self.inner
            .process_qr_string(qr_string)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(WasmQRResult {
            chunk_index: chunk.chunk_index,
            total_chunks: chunk.total_chunks,
        })
    }

    /// True once all chunks for the session have been received.
    #[wasm_bindgen]
    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }

    /// Number of chunks expected in the current session (0 until first chunk seen).
    #[wasm_bindgen]
    pub fn total_count(&self) -> usize {
        self.inner.total_count()
    }

    /// Number of distinct chunks received so far.
    #[wasm_bindgen]
    pub fn received_count(&self) -> usize {
        self.inner.received_count()
    }

    /// Session ID of the current transfer, or `undefined` if no chunk seen yet.
    #[wasm_bindgen]
    pub fn session_id(&self) -> Option<u32> {
        self.inner.session_id()
    }

    /// Reassembled data. Call only after `is_complete()` returns true.
    #[wasm_bindgen]
    pub fn get_data(&self) -> Result<Vec<u8>, JsError> {
        self.inner
            .get_data()
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Reset decoder state to start a new session.
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}