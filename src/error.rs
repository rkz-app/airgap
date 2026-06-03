use core::fmt;
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcLevel {
    L,
    M,
    Q,
    H,
}

#[cfg(not(cbindgen))]
#[derive(Debug)]
pub enum AirgapError {
    UnknownError,
    InvalidMagic,
    UnsupportedVersion(u8),
    CrcMismatch,
    SessionMismatch,
    MetadataMismatch,
    ChunkOutOfBounds(u16),
    TooManyChunks(usize),
    ChunkSizeTooLarge(usize, usize),
    ChunkSizeTooSmall(usize, usize),
    MissingChunk(u16),
    EncodingError(String),
    EmptyData,
}

// When std feature is enabled, derive std::error::Error via thiserror
#[cfg(all(not(cbindgen), feature = "std"))]
impl std::error::Error for AirgapError {}

impl fmt::Display for AirgapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownError => write!(f, "Unknown error"),
            Self::InvalidMagic => write!(f, "Invalid magic bytes"),
            Self::UnsupportedVersion(v) => write!(f, "Unsupported version: {v}"),
            Self::CrcMismatch => write!(f, "CRC mismatch"),
            Self::SessionMismatch => write!(f, "Session ID mismatch"),
            Self::MetadataMismatch => write!(f, "Metadata mismatch"),
            Self::ChunkOutOfBounds(i) => write!(f, "Chunk index {i} out of bounds"),
            Self::TooManyChunks(n) => write!(f, "Too many chunks: {n} (max 65535)"),
            Self::ChunkSizeTooLarge(sz, max) => write!(f, "Chunk size {sz} exceeds maximum {max}"),
            Self::ChunkSizeTooSmall(sz, min) => write!(f, "Chunk size {sz} below minimum {min}"),
            Self::MissingChunk(i) => write!(f, "Missing chunk {i}"),
            Self::EncodingError(e) => write!(f, "Encoding error: {e}"),
            Self::EmptyData => write!(f, "Empty data for encoder"),
        }
    }
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
pub const AIRGAP_ERR_EMPTY_DATA: i32 = -22;

#[cfg(not(cbindgen))]
impl AirgapError {
    pub(crate) fn to_code(&self) -> i32 {
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
            AirgapError::EmptyData => AIRGAP_ERR_EMPTY_DATA,
        }
    }
}
