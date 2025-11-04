use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::blacklist::Blacklist;

pub struct BlpFile {
    pub path: PathBuf,
    pub relative_path: String,
}

pub struct FileCollector {
    global_files: HashMap<String, BlpFile>,
    cn_files: HashMap<String, BlpFile>,
}

impl FileCollector {
    pub fn new() -> Self {
        Self {
            global_files: HashMap::new(),
            cn_files: HashMap::new(),
        }
    }

    pub fn collect_files(
        &mut self,
        global_dir: &Path,
        cn_dir: &Path,
        blacklist: &Blacklist,
    ) -> Result<()> {
        self.global_files = self.collect_blp_files_from_dir(global_dir, blacklist)?;
        self.cn_files = self.collect_blp_files_from_dir(cn_dir, blacklist)?;
        Ok(())
    }

    fn collect_blp_files_from_dir(
        &self,
        dir: &Path,
        blacklist: &Blacklist,
    ) -> Result<HashMap<String, BlpFile>> {
        let mut files = HashMap::new();

        if !dir.exists() {
            return Err(anyhow::anyhow!("Directory does not exist: {:?}", dir));
        }

        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "blp" {
                        // Check if file is in blacklist
                        if blacklist.should_exclude(path) {
                            continue;
                        }

                        if let Ok(relative_path) = path.strip_prefix(dir) {
                            let relative_path_str = relative_path.to_string_lossy().to_lowercase();
                            files.insert(
                                relative_path_str.clone(),
                                BlpFile {
                                    path: path.to_path_buf(),
                                    relative_path: relative_path_str,
                                },
                            );
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    pub fn get_global_files(&self) -> &HashMap<String, BlpFile> {
        &self.global_files
    }

    pub fn get_cn_files(&self) -> &HashMap<String, BlpFile> {
        &self.cn_files
    }

    pub fn get_files_to_compare(&self) -> Vec<String> {
        self.global_files
            .keys()
            .filter(|key| self.cn_files.contains_key(*key))
            .cloned()
            .collect()
    }

    pub fn get_global_only_files(&self) -> Vec<&BlpFile> {
        self.global_files
            .values()
            .filter(|file| !self.cn_files.contains_key(&file.relative_path))
            .collect()
    }

    pub fn get_filtered_count(&self) -> usize {
        self.global_files.len() + self.cn_files.len()
    }
}
