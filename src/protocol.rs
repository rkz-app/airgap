use crate::error::AirgapError;

pub const MAGIC: [u8; 2] = [0x19, 0xF7];
pub const VERSION: u8 = 1;
pub const HEADER_SIZE: usize = 16;
pub const MAX_CHUNK_SIZE: usize = 1920;
pub const RECOMMENDED_MAX_CHUNK_SIZE: usize = 1100;
pub const MIN_CHUNK_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub total_chunks: u16,
    pub chunk_index: u16,
    pub session_id: u32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn new(
        total_chunks: u16,
        chunk_index: u16,
        session_id: u32,
        data: Vec<u8>,
    ) -> Result<Self, AirgapError> {
        // Validate chunk data size
        if data.len() > MAX_CHUNK_SIZE {
            return Err(AirgapError::ChunkSizeTooLarge(
                data.len(),
                MAX_CHUNK_SIZE,
            ));
        }

        if data.is_empty() {
            return Err(AirgapError::ChunkSizeTooSmall(0, 1));
        }

        Ok(Self {
            total_chunks,
            chunk_index,
            session_id,
            data,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16 + self.data.len() + 4);

        bytes.extend_from_slice(&MAGIC);

        bytes.push(VERSION);

        bytes.extend_from_slice(&self.total_chunks.to_be_bytes());

        bytes.extend_from_slice(&self.chunk_index.to_be_bytes());

        bytes.extend_from_slice(&self.session_id.to_be_bytes());

        bytes.extend_from_slice(&(self.data.len() as u16).to_be_bytes());

        bytes.extend_from_slice(&[0, 0, 0]);

        bytes.extend_from_slice(&self.data);

        let crc = crc32fast::hash(&bytes);
        bytes.extend_from_slice(&crc.to_be_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, AirgapError> {

        if bytes.len() < HEADER_SIZE + 4 {
            return Err(AirgapError::EncodingError(
                "Chunk too small".into()
            ));
        }

        if &bytes[0..2] != &MAGIC {
            return Err(AirgapError::InvalidMagic);
        }

        let version = bytes[2];
        if version != VERSION {
            return Err(AirgapError::UnsupportedVersion(version));
        }

        let total_chunks = u16::from_be_bytes([bytes[3], bytes[4]]);
        let chunk_index = u16::from_be_bytes([bytes[5], bytes[6]]);
        let session_id = u32::from_be_bytes([
            bytes[7], bytes[8], bytes[9], bytes[10]
        ]);
        let data_len = u16::from_be_bytes([bytes[11], bytes[12]]) as usize;

        if chunk_index >= total_chunks {
            return Err(AirgapError::ChunkOutOfBounds(chunk_index));
        }

        // Validate data length
        if data_len > MAX_CHUNK_SIZE {
            return Err(AirgapError::ChunkSizeTooLarge(
                data_len,
                MAX_CHUNK_SIZE,
            ));
        }

        let data_start = HEADER_SIZE;
        let data_end = data_start + data_len;

        if bytes.len() < data_end + 4 {
            return Err(AirgapError::EncodingError(
                "Chunk truncated".into()
            ));
        }

        let data = bytes[data_start..data_end].to_vec();

        // Verify CRC
        let stored_crc = u32::from_be_bytes([
            bytes[data_end],
            bytes[data_end + 1],
            bytes[data_end + 2],
            bytes[data_end + 3],
        ]);
        let calculated_crc = crc32fast::hash(&bytes[..data_end]);

        if stored_crc != calculated_crc {
            return Err(AirgapError::CrcMismatch);
        }

        Ok(Self {
            total_chunks,
            chunk_index,
            session_id,
            data,
        })
    }
}