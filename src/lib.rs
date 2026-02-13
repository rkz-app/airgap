pub mod protocol;
pub mod encoder;
pub mod decoder;
pub mod ffi;

#[cfg(target_os = "android")]
pub mod ffi_android;

pub use protocol::{Chunk, TransportError};
pub use encoder::{Encoder, QrConfig};
pub use decoder::Decoder;
pub use qrcode::EcLevel;

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
            let chunk_bytes = encoder.get_chunk_bytes(i);
            let qr_string = base45::encode_from_buffer(chunk_bytes);
            decoder.process_qr_string(&qr_string).unwrap();
        }

        // Decode
        assert!(decoder.is_complete());
        let decoded = decoder.get_data().unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
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