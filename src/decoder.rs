use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::format;
use alloc::string::ToString;
use crate::protocol::*;
use crate::error::AirgapError;

pub struct Decoder {
    received_chunks: BTreeMap<u16, Vec<u8>>,
    availability: Vec<bool>,
    session_id: Option<u32>,
    total_chunks: Option<u16>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            received_chunks: BTreeMap::new(),
            availability: Vec::new(),
            session_id: None,
            total_chunks: None,
        }
    }

    /// Process a scanned QR code string
    pub fn process_qr_string(&mut self, qr_data: &str) -> Result<Chunk, AirgapError> {
        // Decode Base45
        let chunk_bytes = base45::decode(qr_data).map_err(|e| {
            return AirgapError::EncodingError(e.to_string())
        })?;

        // Parse chunk
        let chunk = Chunk::from_bytes(chunk_bytes.as_slice())?;

        // Initialize session on first chunk
        if self.session_id.is_none() {
            self.session_id = Some(chunk.session_id);
            self.total_chunks = Some(chunk.total_chunks);
            // Allocate availability vec lazily
            self.availability = alloc::vec![false; chunk.total_chunks as usize];
        }

        if self.total_chunks.unwrap() != chunk.total_chunks {
            return Err(AirgapError::MetadataMismatch)
        }

        if Some(chunk.session_id) != self.session_id {
            return Err(AirgapError::SessionMismatch);
        }

        // Store chunk data
        self.received_chunks.insert(chunk.chunk_index, chunk.data.clone());
        if let Some(available) = self.availability.get_mut(chunk.chunk_index as usize) {
            *available = true;
        }

        Ok(chunk)
    }

    pub fn is_complete(&self) -> bool {
        match self.total_chunks {
            Some(total) => self.received_chunks.len() == total as usize,
            None => false,
        }
    }

    pub fn session_id(&self) -> Option<u32> {
        self.session_id
    }

    pub fn received_indices(&self) -> impl Iterator<Item = u16> {
        self.received_chunks.keys().copied()
    }

    pub fn received_count(&self) -> usize {
        self.received_chunks.len()
    }

    pub fn total_count(&self) -> usize {
        self.total_chunks.unwrap_or(0) as usize
    }

    /// Check if a specific chunk index has been received.
    /// Returns false before the first chunk is processed (total_chunks unknown).
    pub fn is_available(&self, index: u16) -> bool {
        self.availability.get(index as usize).copied().unwrap_or(false)
    }

    /// Get reassembled data
    pub fn get_data(&self) -> Result<Vec<u8>, AirgapError> {
        if !self.is_complete() {
            return Err(AirgapError::EncodingError(
                format!("Incomplete: {}/{} chunks", self.received_count(), self.total_count())
            ));
        }

        let total_chunks = self.total_chunks.unwrap();

        // Reassemble in order
        let mut result = Vec::new();
        for i in 0..total_chunks {
            let chunk_data = self.received_chunks.get(&i)
                .ok_or(AirgapError::MissingChunk(i))?;
            result.extend_from_slice(chunk_data);
        }

        Ok(result)
    }
    pub fn reset(&mut self) {
        self.received_chunks.clear();
        self.availability.clear();
        self.session_id = None;
        self.total_chunks = None;
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
