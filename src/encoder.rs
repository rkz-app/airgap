// encoder.rs
use crate::error::AirgapError;
use crate::protocol::*;
use qrcode::{QrCode, EcLevel};
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
    config: QrConfig,
}

impl Encoder {
    pub fn new(
        data: &[u8],
        chunk_size: usize,
    ) -> Result<Self, AirgapError> {
        Self::with_config(data, chunk_size, QrConfig::default())
    }

    pub fn with_config(
        data: &[u8],
        chunk_size: usize,
        config: QrConfig,
    ) -> Result<Self, AirgapError> {

        if data.len() == 0 {
            return Err(AirgapError::EmptyData);
        }

        // Validate chunk size
        if chunk_size < MIN_CHUNK_SIZE {
            return Err(AirgapError::ChunkSizeTooSmall(chunk_size, MIN_CHUNK_SIZE));
        }

        if chunk_size > MAX_CHUNK_SIZE {
            return Err(AirgapError::ChunkSizeTooLarge(
                chunk_size,
                MAX_CHUNK_SIZE,
            ));
        }

        // Warn if using very large chunk size (won't scan well)
        if chunk_size > RECOMMENDED_MAX_CHUNK_SIZE {
            eprintln!(
                "Warning: chunk size {} exceeds recommended maximum {}. \
                 QR codes may be difficult to scan.",
                chunk_size, RECOMMENDED_MAX_CHUNK_SIZE
            );
        }

        let total_chunks = (data.len() + chunk_size - 1) / chunk_size;

        if total_chunks > 65535 {
            return Err(AirgapError::TooManyChunks(total_chunks));
        }

        let session_id = rand::random::<u32>();
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
            config,
        })
    }
    pub fn get_qr_string(&self, index: usize) -> Result<String, AirgapError> {
        if (index > self.chunks.len() - 1) {
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
    pub fn generate_png_bytes(&self) -> Result<Vec<Vec<u8>>, AirgapError> {
        let images = generate_images_from_chunks(&self.chunks, &self.config)?;
        generate_pngs_bytes(images)
    }
    pub fn generate_png_bytes_for_item(&self, index: usize) ->  Result<Vec<u8>, AirgapError> {
        if (index > self.chunk_count()) {
            return Err(AirgapError::ChunkOutOfBounds(index as u16))
        }
        let image = generate_image_from_chunk(&self.chunks[index], &self.config)?;
        generate_png_bytes(&image)
    }
}


pub fn generate_image_from_chunk(chunk: &Chunk, config: &QrConfig) -> Result<DynamicImage, AirgapError> {
    let chunk_bytes = chunk.to_bytes();
    let encoded = base45::encode(chunk_bytes);
    let code = QrCode::with_error_correction_level(&encoded, config.ec_level)
        .map_err(|e| AirgapError::EncodingError(e.to_string()))?;

    let image = code.render::<Luma<u8>>()
        .min_dimensions(config.qr_size, config.qr_size)
        .build();
    Ok(DynamicImage::ImageLuma8(image))
}

pub fn generate_images_from_chunks(chunks: &Vec<Chunk>, qr_config: &QrConfig) -> Result<Vec<DynamicImage>, AirgapError> {
    let mut images = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        images.push(generate_image_from_chunk(chunk, qr_config)?)
    }
    Ok(images)
}

pub fn generate_png_bytes(image: &DynamicImage) -> Result<Vec<u8>, AirgapError> {
    let mut bytes = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png
    ).map_err(|e| AirgapError::EncodingError(e.to_string()))?;
    Ok(bytes)
}

pub fn generate_pngs_bytes(images: Vec<DynamicImage>) -> Result<Vec<Vec<u8>>, AirgapError> {
    let mut png_bytes = Vec::with_capacity(images.len());

    for img in &images {
        png_bytes.push(generate_png_bytes(img)?);
    }

    Ok(png_bytes)
}