use anyhow::{Context, Result};
use image::DynamicImage;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use wow_blp::{convert::blp_to_image, parser::load_blp};

pub struct BlpComparator;

impl BlpComparator {
    pub fn new() -> Self {
        Self
    }

    pub fn are_files_different(&self, global_path: &Path, cn_path: &Path) -> Result<bool> {
        // First check file size difference for early exit
        let global_metadata = fs::metadata(global_path).with_context(|| {
            format!("Failed to get metadata for global file: {:?}", global_path)
        })?;
        let cn_metadata = fs::metadata(cn_path)
            .with_context(|| format!("Failed to get metadata for CN file: {:?}", cn_path))?;

        if global_metadata.len() != cn_metadata.len() {
            return Ok(true);
        }

        // Check file hash for early exit
        let global_hash = self.calculate_file_hash(global_path)?;
        let cn_hash = self.calculate_file_hash(cn_path)?;

        if global_hash != cn_hash {
            return Ok(true);
        }

        // If hashes are same, compare actual pixel content
        self.compare_pixel_data(global_path, cn_path)
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        let mut hasher = Sha256::new();
        let mut file = fs::File::open(path)
            .with_context(|| format!("Failed to open file for hashing: {:?}", path))?;

        std::io::copy(&mut file, &mut hasher)
            .with_context(|| format!("Failed to read file for hashing: {:?}", path))?;

        Ok(format!("{:x}", hasher.finalize()))
    }

    fn compare_pixel_data(&self, global_path: &Path, cn_path: &Path) -> Result<bool> {
        // Load both BLP files
        let global_blp = load_blp(global_path)
            .with_context(|| format!("Failed to load global BLP file: {:?}", global_path))?;
        let cn_blp = load_blp(cn_path)
            .with_context(|| format!("Failed to load CN BLP file: {:?}", cn_path))?;

        // Basic dimension check
        if global_blp.header.width != cn_blp.header.width
            || global_blp.header.height != cn_blp.header.height
        {
            return Ok(true);
        }

        // Convert to DynamicImage for pixel comparison
        // Use mipmap level 0 (highest resolution)
        let global_image = blp_to_image(&global_blp, 0)
            .with_context(|| format!("Failed to convert global BLP to image: {:?}", global_path))?;
        let cn_image = blp_to_image(&cn_blp, 0)
            .with_context(|| format!("Failed to convert CN BLP to image: {:?}", cn_path))?;

        // Compare pixel data
        self.compare_images(&global_image, &cn_image)
    }

    fn compare_images(&self, global_img: &DynamicImage, cn_img: &DynamicImage) -> Result<bool> {
        // Ensure both images are RGBA format for consistent comparison
        let global_rgba = global_img.to_rgba8();
        let cn_rgba = cn_img.to_rgba8();

        let global_pixels = global_rgba.as_raw();
        let cn_pixels = cn_rgba.as_raw();

        // Quick size check (should be same dimensions by now)
        if global_pixels.len() != cn_pixels.len() {
            return Ok(true);
        }

        // Compare pixel data
        if global_pixels.len() != cn_pixels.len() {
            return Ok(true);
        }

        // Use chunked comparison for better performance and early exit
        let chunk_size = 1024 * 4; // 1024 pixels at a time (4 bytes per pixel)
        let mut different = false;

        for chunk in global_pixels
            .chunks(chunk_size)
            .zip(cn_pixels.chunks(chunk_size))
        {
            if chunk.0 != chunk.1 {
                different = true;
                break;
            }
        }

        Ok(different)
    }
}
