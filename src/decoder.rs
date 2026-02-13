use crate::protocol::*;
use std::collections::HashMap;

pub struct Decoder {
    received_chunks: HashMap<u16, Vec<u8>>,
    session_id: Option<u32>,
    total_chunks: Option<u16>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            received_chunks: HashMap::new(),
            session_id: None,
            total_chunks: None,
        }
    }

    /// Process a scanned QR code string
    pub fn process_qr_string(&mut self, qr_data: &str) -> Result<Chunk, TransportError> {
        // Decode Base45
        let chunk_bytes = base45::decode(qr_data).map_err(|e| {
            return TransportError::EncodingError(e.to_string())
        })?;

        // Parse chunk
        let chunk = Chunk::from_bytes(chunk_bytes.as_slice())?;

        // Initialize session on first chunk
        if self.session_id.is_none() {
            self.session_id = Some(chunk.session_id);
            self.total_chunks = Some(chunk.total_chunks);
        }

        if self.total_chunks.unwrap() != chunk.total_chunks {
            return Err(TransportError::MetadataMismatch)
        }
        
        if Some(chunk.session_id) != self.session_id {
            return Err(TransportError::SessionMismatch);
        }

        // Store chunk data
        self.received_chunks.insert(chunk.chunk_index, chunk.data.clone());

        Ok(chunk)
    }

    pub fn is_complete(&self) -> bool {
        match self.total_chunks {
            Some(total) => self.received_chunks.len() == total as usize,
            None => false,
        }
    }

    pub fn progress(&self) -> (usize, usize) {
        let received = self.received_chunks.len();
        let total = self.total_chunks.unwrap_or(0) as usize;
        (received, total)
    }

    /// Get reassembled data
    pub fn get_data(&self) -> Result<Vec<u8>, TransportError> {
        if !self.is_complete() {
            let (received, total) = self.progress();
            return Err(TransportError::EncodingError(
                format!("Incomplete: {}/{} chunks", received, total)
            ));
        }

        let total_chunks = self.total_chunks.unwrap();

        // Reassemble in order
        let mut result = Vec::new();
        for i in 0..total_chunks {
            let chunk_data = self.received_chunks.get(&i)
                .ok_or(TransportError::MissingChunk(i))?;
            result.extend_from_slice(chunk_data);
        }

        Ok(result)
    }
    pub fn reset(&mut self) {
        self.received_chunks.clear();
        self.session_id = None;
        self.total_chunks = None;
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}