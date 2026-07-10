// encoder.rs
use alloc::vec::Vec;
use alloc::string::String;
use crate::error::AirgapError;
use crate::error::EcLevel;
use crate::protocol::*;

#[cfg(feature = "qr")]
use qrcode::QrCode;
#[cfg(feature = "qr")]
use image::{DynamicImage, Luma};

#[derive(Debug, Clone)]
pub struct QrConfig {
    pub ec_level: EcLevel,
    pub qr_size: u32,
}
impl QrConfig {
    pub fn with_size(size: u32) -> Self {
        Self {
            ec_level: EcLevel::M,
            qr_size: size,
        }
    }
}

impl Default for QrConfig {
    fn default() -> Self {
        Self {
            ec_level: EcLevel::M,
            qr_size: 400,
        }
    }
}

pub struct Encoder {
    chunks: Vec<Chunk>,
    session_id: u32,
    #[cfg(feature = "qr")]
    config: QrConfig,
}

impl Encoder {
    /// Core constructor with explicit session_id — always available.
    /// Use this on embedded targets where you control entropy.
    pub fn with_session_id(
        data: &[u8],
        chunk_size: usize,
        session_id: u32,
    ) -> Result<Self, AirgapError> {
        if data.is_empty() {
            return Err(AirgapError::EmptyData);
        }

        // Validate chunk size
        if chunk_size < AIRGAP_MIN_CHUNK_SIZE {
            return Err(AirgapError::ChunkSizeTooSmall(chunk_size, AIRGAP_MIN_CHUNK_SIZE));
        }

        if chunk_size > AIRGAP_MAX_CHUNK_SIZE {
            return Err(AirgapError::ChunkSizeTooLarge(
                chunk_size,
                AIRGAP_MAX_CHUNK_SIZE,
            ));
        }

        // Warn if using very large chunk size (won't scan well)
        #[cfg(feature = "std")]
        if chunk_size > AIRGAP_RECOMMENDED_MAX_CHUNK_SIZE {
            eprintln!(
                "Warning: chunk size {} exceeds recommended maximum {}. \
                 QR codes may be difficult to scan.",
                chunk_size, AIRGAP_RECOMMENDED_MAX_CHUNK_SIZE
            );
        }

        let total_chunks = (data.len() + chunk_size - 1) / chunk_size;

        if total_chunks > 65535 {
            return Err(AirgapError::TooManyChunks(total_chunks));
        }

        let mut chunks = Vec::with_capacity(total_chunks);

        for i in 0..total_chunks {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(data.len());
            let chunk_data = data[start..end].to_vec();

            let chunk = Chunk::new(
                total_chunks as u16,
                i as u16,
                session_id,
                chunk_data,
            )?;

            chunks.push(chunk);
        }

        Ok(Self {
            chunks,
            session_id,
            #[cfg(feature = "qr")]
            config: QrConfig::default(),
        })
    }

    /// Standard constructor — generates random session_id.
    /// Only available when `std` feature is enabled (needs rand).
    #[cfg(feature = "std")]
    pub fn new(
        data: &[u8],
        chunk_size: usize,
    ) -> Result<Self, AirgapError> {
        let session_id = rand::random::<u32>();
        Self::with_session_id(data, chunk_size, session_id)
    }

    /// Constructor with full config — generates random session_id.
    /// Only available when `qr` feature is enabled (needs rand + QrConfig).
    #[cfg(feature = "qr")]
    pub fn with_config(
        data: &[u8],
        chunk_size: usize,
        config: QrConfig,
    ) -> Result<Self, AirgapError> {
        let mut encoder = Self::with_session_id(data, chunk_size, rand::random::<u32>())?;
        encoder.config = config;
        Ok(encoder)
    }

    pub fn get_qr_string(&self, index: usize) -> Result<String, AirgapError> {
        if index > self.chunks.len() - 1 {
            return Err(AirgapError::ChunkOutOfBounds(index as u16))
        }
        let chunk_bytes = self.chunks[index].to_bytes();
        Ok(base45::encode(&chunk_bytes))
    }

    pub fn session_id(&self) -> u32 {
        self.session_id
    }
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    #[cfg(feature = "qr")]
    pub fn generate_png_bytes(&self) -> Result<Vec<Vec<u8>>, AirgapError> {
        let images = generate_images_from_chunks(&self.chunks, &self.config)?;
        generate_pngs_bytes(images)
    }

    #[cfg(feature = "qr")]
    pub fn generate_png_bytes_for_item(&self, index: usize) ->  Result<Vec<u8>, AirgapError> {
        if index >= self.chunk_count() {
            return Err(AirgapError::ChunkOutOfBounds(index as u16))
        }
        let image = generate_image_from_chunk(&self.chunks[index], &self.config)?;
        generate_png_bytes(&image)
    }
}


#[cfg(feature = "qr")]
pub fn generate_image_from_chunk(chunk: &Chunk, config: &QrConfig) -> Result<DynamicImage, AirgapError> {
    let chunk_bytes = chunk.to_bytes();
    let encoded = base45::encode(chunk_bytes);
    let ec = match config.ec_level {
        EcLevel::L => qrcode::EcLevel::L,
        EcLevel::M => qrcode::EcLevel::M,
        EcLevel::Q => qrcode::EcLevel::Q,
        EcLevel::H => qrcode::EcLevel::H,
    };
    let code = QrCode::with_error_correction_level(&encoded, ec)
        .map_err(|e| AirgapError::EncodingError(e.to_string()))?;

    let image = code.render::<Luma<u8>>()
        .min_dimensions(config.qr_size, config.qr_size)
        .build();
    Ok(DynamicImage::ImageLuma8(image))
}

#[cfg(feature = "qr")]
pub fn generate_images_from_chunks(chunks: &Vec<Chunk>, qr_config: &QrConfig) -> Result<Vec<DynamicImage>, AirgapError> {
    let mut images = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        images.push(generate_image_from_chunk(chunk, qr_config)?)
    }
    Ok(images)
}

#[cfg(feature = "qr")]
pub fn generate_png_bytes(image: &DynamicImage) -> Result<Vec<u8>, AirgapError> {
    let mut bytes = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png
    ).map_err(|e| AirgapError::EncodingError(e.to_string()))?;
    Ok(bytes)
}

#[cfg(feature = "qr")]
pub fn generate_pngs_bytes(images: Vec<DynamicImage>) -> Result<Vec<Vec<u8>>, AirgapError> {
    let mut png_bytes = Vec::with_capacity(images.len());
    for img in &images {
        png_bytes.push(generate_png_bytes(img)?);
    }
    Ok(png_bytes)
}
