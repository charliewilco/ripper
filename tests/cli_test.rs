use std::fs::File;
use std::io::Write;

use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_cli_find_no_args() {
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("find")
		.assert()
		.failure()
		.stderr(predicate::str::contains("required arguments were not provided"))
		.stderr(predicate::str::contains("Usage: ripper"))
		.stderr(predicate::str::contains("find <PATTERN>"));
}

#[test]
fn test_cli_invalid_command() {
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("invalid").assert().failure();
}

#[test]
fn test_cli_version() {
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("--version").assert().success().stdout(predicate::str::contains("ripper"));
}

#[test]
fn test_cli_help() {
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("--help")
		.assert()
		.success()
		.stdout(predicate::str::contains("Usage:"))
		.stdout(predicate::str::contains("Commands:"))
		.stdout(predicate::str::contains("find"))
		.stdout(predicate::str::contains("delete"));
}

#[test]
fn test_cli_find_lists_matches_without_deleting() {
	let dir = tempdir().unwrap();
	let txt_file = dir.path().join("test.txt");
	File::create(&txt_file).unwrap().write_all(b"test content").unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("find")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.assert()
		.success()
		.stdout(predicate::str::contains("test.txt"))
		.stdout(predicate::str::contains("Successfully deleted").not());

	assert!(txt_file.exists());
}

#[test]
fn test_cli_delete_with_pattern() {
	let dir = tempdir().unwrap();
	let txt_file = dir.path().join("test.txt");
	File::create(&txt_file).unwrap().write_all(b"test content").unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("delete")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.arg("-y")
		.assert()
		.success()
		.stdout(predicate::str::contains("test.txt"))
		.stdout(predicate::str::contains("Successfully deleted"));

	assert!(!txt_file.exists());
}

#[test]
fn test_cli_find_no_matches() {
	let dir = tempdir().unwrap();
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("find")
		.arg(r"nonexistent\.txt$")
		.arg("-d")
		.arg(dir.path())
		.assert()
		.success()
		.stdout(predicate::str::contains("No matching files found"));
}

#[test]
fn test_cli_delete_verbose_output() {
	let dir = tempdir().unwrap();
	let txt_file = dir.path().join("verbose.txt");
	File::create(&txt_file).unwrap().write_all(b"test content").unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("delete")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.arg("-v")
		.arg("-y")
		.assert()
		.success()
		.stdout(predicate::str::contains("Searching for:"))
		.stdout(predicate::str::contains("Starting from:"))
		.stdout(predicate::str::contains("Following links: no"))
		.stdout(predicate::str::contains("Deleting files..."))
		.stdout(predicate::str::contains("Successfully deleted"));
}

#[test]
fn test_cli_delete_prompt_cancel() {
	let dir = tempdir().unwrap();
	let txt_file = dir.path().join("prompt.txt");
	File::create(&txt_file).unwrap().write_all(b"test content").unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("delete")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.write_stdin("n\n")
		.assert()
		.success()
		.stdout(predicate::str::contains("Do you want to delete all these files?"))
		.stdout(predicate::str::contains("Operation cancelled"));

	assert!(txt_file.exists());
}

#[test]
fn test_cli_find_invalid_regex() {
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("find")
		.arg("[")
		.assert()
		.failure()
		.stderr(predicate::str::contains("Invalid regex pattern"));
}

#[test]
fn test_cli_delete_invalid_regex() {
	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("delete")
		.arg("[")
		.arg("-y")
		.assert()
		.failure()
		.stderr(predicate::str::contains("Invalid regex pattern"));
}

#[cfg(unix)]
#[test]
fn test_cli_delete_exits_non_zero_on_partial_failure() {
	use std::fs;
	use std::os::unix::fs::PermissionsExt;

	let dir = tempdir().unwrap();
	let deletable_file = dir.path().join("deletable.txt");
	let protected_dir = dir.path().join("protected");
	let protected_file = protected_dir.join("protected.txt");

	File::create(&deletable_file).unwrap().write_all(b"deletable").unwrap();
	fs::create_dir(&protected_dir).unwrap();
	File::create(&protected_file).unwrap().write_all(b"protected").unwrap();

	let mut perms = fs::metadata(&protected_dir).unwrap().permissions();
	perms.set_mode(0o555);
	fs::set_permissions(&protected_dir, perms).unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("delete")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.arg("-y")
		.assert()
		.failure()
		.stdout(predicate::str::contains("Successfully deleted"))
		.stdout(predicate::str::contains("Errors:"))
		.stderr(predicate::str::contains("Failed to delete 1 out of 2 matching files"));

	let mut perms = fs::metadata(&protected_dir).unwrap().permissions();
	perms.set_mode(0o755);
	fs::set_permissions(&protected_dir, perms).unwrap();

	assert!(!deletable_file.exists());
	assert!(protected_file.exists());
}
