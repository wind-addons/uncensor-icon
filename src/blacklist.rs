use std::collections::HashSet;
use std::path::Path;

pub struct Blacklist {
    excluded_files: HashSet<String>,
}

impl Blacklist {
    pub fn new(blacklist_str: &str) -> Self {
        let mut excluded_files = HashSet::new();

        // Split by comma and trim whitespace
        for file in blacklist_str.split(',') {
            let file_name = file.trim().to_lowercase();
            if !file_name.is_empty() {
                excluded_files.insert(file_name);
            }
        }

        Self { excluded_files }
    }

    pub fn should_exclude(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name() {
            let file_name_str = file_name.to_string_lossy().to_lowercase();
            return self.excluded_files.contains(&file_name_str);
        }
        false
    }

    pub fn get_excluded_count(&self) -> usize {
        self.excluded_files.len()
    }

    pub fn get_excluded_files(&self) -> Vec<String> {
        self.excluded_files.iter().cloned().collect()
    }
}
