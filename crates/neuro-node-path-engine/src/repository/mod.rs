pub mod indexer;
pub mod code_analyzer;
pub mod model_loader;

pub use indexer::Indexer;
pub use code_analyzer::CodeAnalyzer;

use std::path::PathBuf;
use walkdir::WalkDir;

pub struct CodeRepository {
    root_path: PathBuf,
}

impl CodeRepository {
    pub fn new(root_path: PathBuf) -> anyhow::Result<Self> {
        if !root_path.exists() {
            anyhow::bail!("Repository path does not exist: {:?}", root_path);
        }
        Ok(Self { root_path })
    }

    pub async fn scan_files(&self) -> anyhow::Result<Vec<String>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if let Some(path) = entry.path().to_str() {
                files.push(path.to_string());
            }
        }

        Ok(files)
    }

    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }
}
