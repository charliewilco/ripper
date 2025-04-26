use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use grep_regex::RegexMatcher;
use walkdir::WalkDir;

/// Represents a file found by the search
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundFile {
    pub path: PathBuf,
}

impl FoundFile {
    /// Create a new FoundFile
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Delete the file
    pub fn delete(&self) -> Result<()> {
        fs::remove_file(&self.path).context("Failed to delete file")
    }

    /// Check if the file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

/// Find files matching a pattern in a directory
pub fn find_files<P: AsRef<Path>>(pattern: &str, start_dir: P) -> Result<Vec<FoundFile>> {
    let matcher = RegexMatcher::new(pattern)
        .context("Invalid regex pattern")?;
    
    let mut file_list = Vec::new();
    
    for entry in WalkDir::new(start_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        
        let path = entry.path();
        
        // Skip directories
        if path.is_dir() {
            continue;
        }
        
        // Check if file name matches the pattern
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if matcher.is_match(file_name_str) {
                    file_list.push(FoundFile::new(path));
                }
            }
        }
    }
    
    Ok(file_list)
}

/// Batch delete files
pub fn delete_files(files: &[FoundFile]) -> (usize, Vec<(PathBuf, String)>) {
    let mut deleted_count = 0;
    let mut errors = Vec::new();
    
    for file in files {
        match file.delete() {
            Ok(_) => {
                deleted_count += 1;
            },
            Err(e) => {
                errors.push((file.path.clone(), e.to_string()));
            },
        }
    }
    
    (deleted_count, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_found_file_new() {
        let path = Path::new("/tmp/test.txt");
        let found_file = FoundFile::new(path);
        assert_eq!(found_file.path, path);
    }

    #[test]
    fn test_found_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        
        // File should not exist yet
        let found_file = FoundFile::new(&file_path);
        assert!(!found_file.exists());
        
        // Create the file
        let mut file = File::create(&file_path).unwrap();
        write!(file, "test content").unwrap();
        
        // Now it should exist
        assert!(found_file.exists());
    }

    #[test]
    fn test_found_file_delete() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        
        // Create a file
        let mut file = File::create(&file_path).unwrap();
        write!(file, "test content").unwrap();
        
        // Make sure it exists
        assert!(file_path.exists());
        
        // Delete it
        let found_file = FoundFile::new(&file_path);
        found_file.delete().unwrap();
        
        // It should no longer exist
        assert!(!file_path.exists());
    }

    #[test]
    fn test_find_files() {
        let dir = tempdir().unwrap();
        
        // Create some test files
        let file1 = dir.path().join("test1.txt");
        let file2 = dir.path().join("test2.txt");
        let file3 = dir.path().join("other.log");
        
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();
        
        // Find all .txt files
        let results = find_files(r"\.txt$", dir.path()).unwrap();
        
        // Should find 2 .txt files
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|f| f.path == file1));
        assert!(results.iter().any(|f| f.path == file2));
        assert!(!results.iter().any(|f| f.path == file3));
    }

    #[test]
    fn test_delete_files() {
        let dir = tempdir().unwrap();
        
        // Create some test files
        let file1 = dir.path().join("delete1.txt");
        let file2 = dir.path().join("delete2.txt");
        
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        
        // Create found files
        let found_files = vec![
            FoundFile::new(&file1),
            FoundFile::new(&file2),
        ];
        
        // Delete them
        let (deleted_count, errors) = delete_files(&found_files);
        
        // Both should have been deleted successfully
        assert_eq!(deleted_count, 2);
        assert_eq!(errors.len(), 0);
        assert!(!file1.exists());
        assert!(!file2.exists());
    }

    #[test]
    fn test_delete_files_with_error() {
        let dir = tempdir().unwrap();
        
        // Create one file to delete
        let file1 = dir.path().join("delete1.txt");
        File::create(&file1).unwrap();
        
        // Try to delete one file that doesn't exist
        let file2 = dir.path().join("nonexistent.txt");
        
        // Create found files
        let found_files = vec![
            FoundFile::new(&file1),
            FoundFile::new(&file2),
        ];
        
        // Attempt to delete them
        let (deleted_count, errors) = delete_files(&found_files);
        
        // One should be deleted, one should error
        assert_eq!(deleted_count, 1);
        assert_eq!(errors.len(), 1);
        assert!(!file1.exists());
        assert_eq!(errors[0].0, file2);
    }
}
