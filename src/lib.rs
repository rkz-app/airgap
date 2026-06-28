#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// Provide system allocator for host-based no_std builds (testing/development)
// Embedded targets (target_os = "none") provide their own #[global_allocator]
#[cfg(all(
    not(feature = "std"),
    not(target_arch = "wasm32"),
    not(target_os = "none")
))]
extern crate std;
#[cfg(all(
    not(feature = "std"),
    not(target_arch = "wasm32"),
    not(target_os = "none")
))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

// Stub allocator + panic handler for CI builds targeting embedded.
// Produces a linkable .a — real firmware must provide its own.
#[cfg(all(
    not(feature = "std"),
    not(target_arch = "wasm32"),
    target_os = "none"
))]
mod ci_stubs {
    use core::alloc::{GlobalAlloc, Layout};
    use core::panic::PanicInfo;

    struct CiAlloc;
    unsafe impl GlobalAlloc for CiAlloc {
        unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
            core::ptr::null_mut()
        }
        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
    }
    #[global_allocator]
    static GLOBAL: CiAlloc = CiAlloc;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

pub mod protocol;
pub mod encoder;
pub mod decoder;
pub mod ffi;
#[cfg(feature = "std")]
pub mod ffi_android;  // JNI bindings for all JVM targets (Android, desktop Java/Kotlin)
#[cfg(all(feature = "std", target_arch = "wasm32"))]
pub mod ffi_wasm;
mod error;
mod c_result;

pub use protocol::Chunk;
pub use encoder::{Encoder, QrConfig};
pub use decoder::Decoder;
pub use error::EcLevel;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        // Arbitrary data
        let data = vec![0x42; 2000]; // 2KB of data

        // Encode
        let encoder = Encoder::new(&data, 500).unwrap();
        println!("Chunks: {}", encoder.chunk_count());

        // Simulate scanning
        let mut decoder = Decoder::new();

        // Get raw encoded strings (simulate QR scanning)
        for i in 0..encoder.chunk_count() {
            let qr_string = encoder.get_qr_string(i).unwrap();
            decoder.process_qr_string(&qr_string).unwrap();
        }

        // Decode
        assert!(decoder.is_complete());
        let decoded = decoder.get_data().unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_received_indices() {
        let data = vec![0xAB; 1500];
        let encoder = Encoder::new(&data, 500).unwrap();
        assert_eq!(encoder.chunk_count(), 3);

        let mut decoder = Decoder::new();

        // Scan first two chunks, skip the last
        decoder.process_qr_string(&encoder.get_qr_string(0).unwrap()).unwrap();
        decoder.process_qr_string(&encoder.get_qr_string(1).unwrap()).unwrap();

        assert!(!decoder.is_complete());

        let indices: Vec<u16> = decoder.received_indices().collect();
        assert_eq!(indices, vec![0, 1]);

        // Scan the last chunk
        decoder.process_qr_string(&encoder.get_qr_string(2).unwrap()).unwrap();
        assert!(decoder.is_complete());

        let all_indices: Vec<u16> = decoder.received_indices().collect();
        assert_eq!(all_indices, vec![0, 1, 2]);

        let decoded = decoder.get_data().unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_received_indices_gaps() {
        let data = vec![0xCD; 800];
        let encoder = Encoder::new(&data, 200).unwrap();
        assert_eq!(encoder.chunk_count(), 4);

        let mut decoder = Decoder::new();

        // Scan chunks out of order with a gap
        decoder.process_qr_string(&encoder.get_qr_string(3).unwrap()).unwrap();
        decoder.process_qr_string(&encoder.get_qr_string(0).unwrap()).unwrap();
        decoder.process_qr_string(&encoder.get_qr_string(1).unwrap()).unwrap();

        let indices: Vec<u16> = decoder.received_indices().collect();
        assert_eq!(indices, vec![0, 1, 3]);

        // Fill the gap
        decoder.process_qr_string(&encoder.get_qr_string(2).unwrap()).unwrap();
        assert!(decoder.is_complete());

        let decoded = decoder.get_data().unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    #[cfg(feature = "qr")]
    fn test_ml_kem_key() {

        let pubkey = vec![0xAB; 1568];

        let encoder = Encoder::new(&pubkey, 780).unwrap();

        assert_eq!(encoder.chunk_count(), 3);

        let pngs = encoder.generate_png_bytes().unwrap();
        assert_eq!(pngs.len(), 3);

        for png in pngs {
            assert!(png.len() > 1000); // PNG has overhead
        }
    }
}
