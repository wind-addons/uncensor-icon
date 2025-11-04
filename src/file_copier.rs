use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileCopier {
    progress: Option<ProgressBar>,
}

impl FileCopier {
    pub fn with_progress(total: u64) -> Self {
        let progress = ProgressBar::new(total);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("📋 {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .expect("Invalid progress template")
                .progress_chars("#>-"),
        );

        Self {
            progress: Some(progress),
        }
    }

    pub fn copy_files(&self, files_to_copy: &[PathBuf], output_dir: &Path) -> Result<u32> {
        let copied_count = std::sync::atomic::AtomicU32::new(0);

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

        // Copy files in parallel
        files_to_copy
            .par_iter()
            .try_for_each(|source_path| -> Result<()> {
                let result = self.copy_single_file(source_path, output_dir);

                if let Some(ref progress) = self.progress {
                    progress.inc(1);
                }

                if result.is_ok() {
                    copied_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                result
            })?;

        Ok(copied_count.load(std::sync::atomic::Ordering::Relaxed))
    }

    fn copy_single_file(&self, source_path: &Path, output_dir: &Path) -> Result<()> {
        let file_name = source_path
            .file_name()
            .with_context(|| format!("Invalid file name: {:?}", source_path))?;

        let dest_path = output_dir.join(file_name);

        // Copy file, overwriting if it already exists
        fs::copy(source_path, &dest_path).with_context(|| {
            format!(
                "Failed to copy file from {:?} to {:?}",
                source_path, dest_path
            )
        })?;

        Ok(())
    }

    pub fn finish_progress(&self) {
        if let Some(ref progress) = self.progress {
            progress.finish();
        }
    }
}
