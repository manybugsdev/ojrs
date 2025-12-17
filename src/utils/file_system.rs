use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn create_directory(path: &str) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory: {}", path))
}

pub fn write_file(path: &str, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path))
}

pub fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn list_files(directory: &str, extension: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();
    
    for entry in fs::read_dir(directory)
        .with_context(|| format!("Failed to read directory: {}", directory))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == extension {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
            }
        }
    }
    
    files.sort();
    Ok(files)
}
