use std::fs::File;
use std::io::Write;

use ripper::{delete_files, find_files, find_files_with_options, FoundFile, SearchOptions};
use tempfile::tempdir;

#[test]
fn test_end_to_end_file_finding() {
	let dir = tempdir().unwrap();

	let txt_file = dir.path().join("document.txt");
	let log_file = dir.path().join("server.log");
	let cfg_file = dir.path().join("config.cfg");
	let ds_store = dir.path().join(".DS_Store");

	File::create(&txt_file).unwrap().write_all(b"text content").unwrap();
	File::create(&log_file).unwrap().write_all(b"log content").unwrap();
	File::create(&cfg_file).unwrap().write_all(b"config content").unwrap();
	File::create(&ds_store).unwrap().write_all(b"DS_Store content").unwrap();

	let subdir = dir.path().join("subdir");
	std::fs::create_dir(&subdir).unwrap();

	let subdir_txt = subdir.join("nested.txt");
	let subdir_ds_store = subdir.join(".DS_Store");

	File::create(&subdir_txt).unwrap().write_all(b"nested content").unwrap();
	File::create(&subdir_ds_store).unwrap().write_all(b"nested DS_Store").unwrap();

	let txt_files = find_files(r"\.txt$", dir.path()).unwrap();
	assert_eq!(txt_files.len(), 2);
	assert!(txt_files.iter().any(|file| file.path == txt_file));
	assert!(txt_files.iter().any(|file| file.path == subdir_txt));

	let ds_files = find_files(r"\.DS_Store$", dir.path()).unwrap();
	assert_eq!(ds_files.len(), 2);
	assert!(ds_files.iter().any(|file| file.path == ds_store));
	assert!(ds_files.iter().any(|file| file.path == subdir_ds_store));

	let all_files_with_ext = find_files(r"\.\w+$", dir.path()).unwrap();
	assert_eq!(all_files_with_ext.len(), 6);

	let hidden_files = find_files(r"^\.", dir.path()).unwrap();
	assert_eq!(hidden_files.len(), 2);
}

#[test]
fn test_end_to_end_delete_operation() {
	let dir = tempdir().unwrap();
	let paths = [
		dir.path().join(".DS_Store"),
		dir.path().join("subdir1").join(".DS_Store"),
		dir.path().join("subdir2").join(".DS_Store"),
	];

	std::fs::create_dir_all(dir.path().join("subdir1")).unwrap();
	std::fs::create_dir_all(dir.path().join("subdir2")).unwrap();

	for path in &paths {
		File::create(path).unwrap().write_all(b"DS_Store content").unwrap();
		assert!(path.exists());
	}

	let other_file = dir.path().join("important.txt");
	File::create(&other_file).unwrap().write_all(b"important content").unwrap();

	let ds_files = find_files(r"\.DS_Store$", dir.path()).unwrap();
	assert_eq!(ds_files.len(), 3);

	let (deleted_count, errors) = delete_files(&ds_files);

	assert_eq!(deleted_count, 3);
	assert_eq!(errors.len(), 0);

	for path in &paths {
		assert!(!path.exists());
	}

	assert!(other_file.exists());

	let remaining_ds_files = find_files(r"\.DS_Store$", dir.path()).unwrap();
	assert_eq!(remaining_ds_files.len(), 0);
}

#[test]
fn test_error_handling_invalid_regex() {
	let result = find_files(r"[", ".");
	assert!(result.is_err());

	let error_message = result.unwrap_err().to_string();
	assert!(error_message.contains("Invalid regex pattern"));
}

#[test]
fn test_error_handling_file_deletion() {
	let dir = tempdir().unwrap();
	let file_path = dir.path().join("test.txt");
	File::create(&file_path).unwrap().write_all(b"test content").unwrap();

	let nonexistent_path = dir.path().join("nonexistent.txt");
	let files = vec![FoundFile::new(&file_path), FoundFile::new(&nonexistent_path)];

	let (deleted, errors) = delete_files(&files);

	assert_eq!(deleted, 1);
	assert_eq!(errors.len(), 1);
	assert_eq!(errors[0].0, nonexistent_path);
	assert!(!file_path.exists());
}

#[cfg(unix)]
#[test]
fn test_find_files_does_not_follow_symlinked_directories_by_default() {
	use std::os::unix::fs::symlink;

	let dir = tempdir().unwrap();
	let external_dir = tempdir().unwrap();
	let external_file = external_dir.path().join("external.txt");
	File::create(&external_file).unwrap().write_all(b"external").unwrap();

	let linked_dir = dir.path().join("linked");
	symlink(external_dir.path(), &linked_dir).unwrap();

	let results = find_files(r"\.txt$", dir.path()).unwrap();

	assert!(!results.iter().any(|file| file.path == external_file));
}

#[cfg(unix)]
#[test]
fn test_find_files_can_follow_symlinked_directories_when_enabled() {
	use std::os::unix::fs::symlink;

	let dir = tempdir().unwrap();
	let external_dir = tempdir().unwrap();
	let external_file = external_dir.path().join("external.txt");
	File::create(&external_file).unwrap().write_all(b"external").unwrap();

	let linked_dir = dir.path().join("linked");
	symlink(external_dir.path(), &linked_dir).unwrap();
	let linked_file = linked_dir.join("external.txt");

	let results =
		find_files_with_options(r"\.txt$", dir.path(), SearchOptions { follow_links: true })
			.unwrap();

	assert!(results.iter().any(|file| file.path == linked_file));
}

#[cfg(unix)]
#[test]
fn test_find_files_returns_error_for_unreadable_directory() {
	use std::fs;
	use std::os::unix::fs::PermissionsExt;

	let dir = tempdir().unwrap();
	let restricted_dir = dir.path().join("restricted");
	std::fs::create_dir(&restricted_dir).unwrap();

	let mut perms = fs::metadata(&restricted_dir).unwrap().permissions();
	perms.set_mode(0o000);
	fs::set_permissions(&restricted_dir, perms).unwrap();

	let result = find_files(r"\.txt$", dir.path());

	let mut perms = fs::metadata(&restricted_dir).unwrap().permissions();
	perms.set_mode(0o755);
	fs::set_permissions(&restricted_dir, perms).unwrap();

	assert!(result.is_err());
}

#[cfg(unix)]
#[test]
fn test_following_broken_symlink_returns_error() {
	use std::os::unix::fs::symlink;

	let dir = tempdir().unwrap();
	let missing_target = dir.path().join("missing");
	let broken_link = dir.path().join("broken");
	symlink(&missing_target, &broken_link).unwrap();

	let result = find_files_with_options(r".*", dir.path(), SearchOptions { follow_links: true });

	assert!(result.is_err());
}

#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn test_find_files_matches_non_utf8_filenames() {
	use std::ffi::OsString;
	use std::os::unix::ffi::OsStringExt;

	let dir = tempdir().unwrap();
	let file_name = OsString::from_vec(vec![b'f', b'o', 0x80, b'.', b't', b'x', b't']);
	let file_path = dir.path().join(&file_name);

	File::create(&file_path).unwrap().write_all(b"test content").unwrap();

	let results = find_files(r"\.txt$", dir.path()).unwrap();

	assert!(results.iter().any(|file| file.path == file_path));
}

#[cfg(unix)]
#[test]
fn test_error_handling_permission_denied() {
	use std::fs;
	use std::os::unix::fs::PermissionsExt;

	let dir = tempdir().unwrap();
	let file_path = dir.path().join("protected.txt");
	File::create(&file_path).unwrap().write_all(b"test content").unwrap();

	let mut perms = fs::metadata(dir.path()).unwrap().permissions();
	perms.set_mode(0o555);
	fs::set_permissions(dir.path(), perms).unwrap();

	let files = vec![FoundFile::new(&file_path)];
	let (deleted, errors) = delete_files(&files);

	assert_eq!(deleted, 0);
	assert_eq!(errors.len(), 1);

	let mut perms = fs::metadata(dir.path()).unwrap().permissions();
	perms.set_mode(0o755);
	fs::set_permissions(dir.path(), perms).unwrap();
}
