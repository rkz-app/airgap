#[derive(Debug, thiserror::Error)]
pub enum AirgapError {
    #[error("Unknown error")]
    UnknownError,
    #[error("Invalid magic bytes")]
    InvalidMagic,
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u8),
    #[error("CRC mismatch")]
    CrcMismatch,
    #[error("Session ID mismatch")]
    SessionMismatch,
    #[error("Metadata mismatch")]
    MetadataMismatch,
    #[error("Chunk index {0} out of bounds")]
    ChunkOutOfBounds(u16),
    #[error("Too many chunks: {0} (max 65535)")]
    TooManyChunks(usize),
    #[error("Chunk size {0} exceeds maximum {1}")]
    ChunkSizeTooLarge(usize, usize),
    #[error("Chunk size {0} below minimum {1}")]
    ChunkSizeTooSmall(usize, usize),
    #[error("Missing chunk {0}")]
    MissingChunk(u16),
    #[error("Encoding error: {0}")]
    EncodingError(String),
}

pub const AIRGAP_UNKNOWN_ERR: i32 = -10;
pub const AIRGAP_ERR_INVALID_MAGIC: i32 = -11;
pub const AIRGAP_ERR_UNSUPPORTED_VERSION: i32 = -12;
pub const AIRGAP_ERR_CRC_MISMATCH: i32 = -13;
pub const AIRGAP_ERR_SESSION_MISMATCH: i32 = -14;
pub const AIRGAP_ERR_METADATA_MISMATCH: i32 = -15;
pub const AIRGAP_ERR_CHUNK_OUT_OF_BOUNDS: i32 = -16;
pub const AIRGAP_ERR_TOO_MANY_CHUNKS: i32 = -17;
pub const AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE: i32 = -18;
pub const AIRGAP_ERR_CHUNK_SIZE_TOO_SMALL: i32 = -19;
pub const AIRGAP_ERR_MISSING_CHUNK: i32 = -20;
pub const AIRGAP_ERR_ENCODING: i32 = -21;

impl AirgapError {
    pub(crate) fn to_code(&self: AirgapError) -> i32 {
        match self {
            AirgapError::UnknownError => AIRGAP_UNKNOWN_ERR,
            AirgapError::InvalidMagic => AIRGAP_ERR_INVALID_MAGIC,
            AirgapError::UnsupportedVersion(_) => AIRGAP_ERR_UNSUPPORTED_VERSION,
            AirgapError::CrcMismatch => AIRGAP_ERR_CRC_MISMATCH,
            AirgapError::MetadataMismatch => AIRGAP_ERR_METADATA_MISMATCH,
            AirgapError::SessionMismatch => AIRGAP_ERR_SESSION_MISMATCH,
            AirgapError::ChunkOutOfBounds(_) => AIRGAP_ERR_CHUNK_OUT_OF_BOUNDS,
            AirgapError::TooManyChunks(_) => AIRGAP_ERR_TOO_MANY_CHUNKS,
            AirgapError::ChunkSizeTooLarge(_, _) => AIRGAP_ERR_CHUNK_SIZE_TOO_LARGE,
            AirgapError::ChunkSizeTooSmall(_, _) => AIRGAP_ERR_CHUNK_SIZE_TOO_SMALL,
            AirgapError::MissingChunk(_) => AIRGAP_ERR_MISSING_CHUNK,
            AirgapError::EncodingError(_) => AIRGAP_ERR_ENCODING,
        }
    }

}