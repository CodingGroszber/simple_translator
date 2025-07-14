use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Handles file operations for our translator
pub struct FileHandler {
    file_path: PathBuf,
}

impl FileHandler {
    /// Create a new file handler for the given file path
    pub fn new(file_path: &Path) -> Self {
        Self {
            file_path: file_path.to_path_buf(),
        }
    }

    /// Read the contents of the file
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(&self.file_path)
            .context(format!("Could not read file: {:?}", self.file_path))
    }

    /// Write the translated content to a new file with appropriate naming
    pub fn write_translated(&self, content: &str, target_lang: &str) -> Result<()> {
        // Get the original file name without extension
        let file_stem = self
            .file_path
            .file_stem()
            .context("Could not extract file name")?
            .to_string_lossy();

        // Get the file extension
        let extension = self
            .file_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy();

        // Create new file path with _translated suffix
        let mut new_path = self.file_path.clone();
        new_path.set_file_name(format!(
            "{}_{}_translated.{}",
            file_stem, target_lang, extension
        ));

        // Write the translated content to the new file
        fs::write(&new_path, content)
            .context(format!("Could not write to file: {:?}", new_path))?;

        println!("Translated file saved to: {:?}", new_path);
        Ok(())
    }
}
