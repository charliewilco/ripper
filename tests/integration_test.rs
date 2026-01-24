use std::fs::File;
use std::io::Write;

use ripper::{delete_files, find_files, FoundFile};
use tempfile::tempdir;

#[test]
fn test_end_to_end_file_finding() {
	let dir = tempdir().unwrap();

	// Create multiple test files with different extensions
	let txt_file = dir.path().join("document.txt");
	let log_file = dir.path().join("server.log");
	let cfg_file = dir.path().join("config.cfg");
	let ds_store = dir.path().join(".DS_Store");

	File::create(&txt_file).unwrap().write_all(b"text content").unwrap();
	File::create(&log_file).unwrap().write_all(b"log content").unwrap();
	File::create(&cfg_file).unwrap().write_all(b"config content").unwrap();
	File::create(&ds_store).unwrap().write_all(b"DS_Store content").unwrap();

	// Create a subdirectory with more files
	let subdir = dir.path().join("subdir");
	std::fs::create_dir(&subdir).unwrap();

	let subdir_txt = subdir.join("nested.txt");
	let subdir_ds_store = subdir.join(".DS_Store");

	File::create(&subdir_txt).unwrap().write_all(b"nested content").unwrap();
	File::create(&subdir_ds_store).unwrap().write_all(b"nested DS_Store").unwrap();

	// Test finding only txt files
	let txt_files = find_files(r"\.txt$", dir.path()).unwrap();
	assert_eq!(txt_files.len(), 2);
	assert!(txt_files.iter().any(|f| f.path == txt_file));
	assert!(txt_files.iter().any(|f| f.path == subdir_txt));

	// Test finding .DS_Store files
	let ds_files = find_files(r"\.DS_Store$", dir.path()).unwrap();
	assert_eq!(ds_files.len(), 2);
	assert!(ds_files.iter().any(|f| f.path == ds_store));
	assert!(ds_files.iter().any(|f| f.path == subdir_ds_store));

	// Test finding all files with any extension (matches .DS_Store too)
	let all_files_with_ext = find_files(r"\.\w+$", dir.path()).unwrap();
	assert_eq!(all_files_with_ext.len(), 6); // txt, log, cfg, txt in subdir, and two .DS_Store

	// Test finding all hidden files
	let hidden_files = find_files(r"^\.", dir.path()).unwrap();
	assert_eq!(hidden_files.len(), 2); // Both .DS_Store files
}

#[test]
fn test_end_to_end_delete_operation() {
	let dir = tempdir().unwrap();

	// Create a few .DS_Store files
	let paths = [
		dir.path().join(".DS_Store"),
		dir.path().join("subdir1").join(".DS_Store"),
		dir.path().join("subdir2").join(".DS_Store"),
	];

	// Create necessary directories
	std::fs::create_dir_all(dir.path().join("subdir1")).unwrap();
	std::fs::create_dir_all(dir.path().join("subdir2")).unwrap();

	// Create the files
	for path in &paths {
		File::create(path).unwrap().write_all(b"DS_Store content").unwrap();
		assert!(path.exists());
	}

	// Create some other files that shouldn't be deleted
	let other_file = dir.path().join("important.txt");
	File::create(&other_file).unwrap().write_all(b"important content").unwrap();

	// Find the .DS_Store files
	let ds_files = find_files(r"\.DS_Store$", dir.path()).unwrap();
	assert_eq!(ds_files.len(), 3);

	// Delete them
	let (deleted_count, errors) = delete_files(&ds_files);

	// Verify all were deleted
	assert_eq!(deleted_count, 3);
	assert_eq!(errors.len(), 0);

	for path in &paths {
		assert!(!path.exists());
	}

	// Make sure the other file wasn't deleted
	assert!(other_file.exists());

	// Verify that a new search finds no .DS_Store files
	let remaining_ds_files = find_files(r"\.DS_Store$", dir.path()).unwrap();
	assert_eq!(remaining_ds_files.len(), 0);
}

#[test]
fn test_error_handling_invalid_regex() {
	// Test that invalid regex patterns return appropriate errors
	let result = find_files(r"[", "."); // Invalid regex pattern
	assert!(result.is_err());

	let error_message = result.unwrap_err().to_string();
	assert!(error_message.contains("Invalid regex pattern"));
}

#[test]
fn test_error_handling_file_deletion() {
	// Create a file
	let dir = tempdir().unwrap();
	let file_path = dir.path().join("test.txt");
	File::create(&file_path).unwrap().write_all(b"test content").unwrap();

	// Create a non-existent file
	let nonexistent_path = dir.path().join("nonexistent.txt");

	// Try to delete both
	let files = vec![FoundFile::new(&file_path), FoundFile::new(&nonexistent_path)];

	let (deleted, errors) = delete_files(&files);

	// One should succeed, one should fail
	assert_eq!(deleted, 1);
	assert_eq!(errors.len(), 1);
	assert_eq!(errors[0].0, nonexistent_path);
	assert!(!file_path.exists());
}
