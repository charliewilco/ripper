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
		.stdout(predicate::str::contains("Options:"));
}

#[test]
fn test_cli_find_with_pattern() {
	let dir = tempdir().unwrap();

	// Create some test files
	let txt_file = dir.path().join("test.txt");
	File::create(&txt_file).unwrap().write_all(b"test content").unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	// Run with the -y flag to bypass confirmation
	cmd.arg("find")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.arg("-y") // Auto-confirm
		.assert()
		.success()
		.stdout(predicate::str::contains("test.txt"))
		.stdout(predicate::str::contains("Successfully deleted"));

	// Verify the file was deleted
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
fn test_cli_verbose_output() {
	let dir = tempdir().unwrap();

	// Create a test file
	let txt_file = dir.path().join("verbose.txt");
	File::create(&txt_file).unwrap().write_all(b"test content").unwrap();

	let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ripper");

	cmd.arg("find")
		.arg(r"\.txt$")
		.arg("-d")
		.arg(dir.path())
		.arg("-v") // Verbose mode
		.arg("-y") // Auto-confirm
		.assert()
		.success()
		.stdout(predicate::str::contains("Searching for:"))
		.stdout(predicate::str::contains("Starting from:"))
		.stdout(predicate::str::contains("Deleting files..."))
		.stdout(predicate::str::contains("Successfully deleted"));
}
