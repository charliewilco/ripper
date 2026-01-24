#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use ripper::{delete_files, find_files, FoundFile};
use tempfile::tempdir;
use test::Bencher;

fn setup_test_directory(dir_path: &Path, num_files: usize) -> Vec<std::path::PathBuf> {
	let mut created_files = Vec::with_capacity(num_files);

	// Create various file types
	for i in 0..num_files {
		let file_path = match i % 4 {
			0 => dir_path.join(format!("file_{}.txt", i)),
			1 => dir_path.join(format!("file_{}.log", i)),
			2 => dir_path.join(format!(".DS_Store_{}", i)),
			_ => dir_path.join(format!("file_{}.cfg", i)),
		};

		File::create(&file_path).unwrap().write_all(b"benchmark test content").unwrap();
		created_files.push(file_path);
	}

	created_files
}

#[bench]
fn bench_find_files_small(b: &mut Bencher) {
	let dir = tempdir().unwrap();
	let _files = setup_test_directory(dir.path(), 50);

	b.iter(|| {
		let results = find_files(r"\.txt$", dir.path()).unwrap();
		// Should find about 1/4 of the files (those with .txt extension)
		assert!(results.len() > 10);
	});
}

#[bench]
fn bench_find_files_medium(b: &mut Bencher) {
	let dir = tempdir().unwrap();
	let _files = setup_test_directory(dir.path(), 500);

	b.iter(|| {
		let results = find_files(r"\.txt$", dir.path()).unwrap();
		// Should find about 1/4 of the files (those with .txt extension)
		assert!(results.len() > 100);
	});
}

#[bench]
fn bench_find_complex_pattern(b: &mut Bencher) {
	let dir = tempdir().unwrap();
	let _files = setup_test_directory(dir.path(), 200);

	// More complex regex with alternation
	b.iter(|| {
		let results = find_files(r"\.txt$|\.log$|\.DS_Store", dir.path()).unwrap();
		// Should find about 3/4 of the files (txt, log, and DS_Store)
		assert!(results.len() > 100);
	});
}

#[bench]
fn bench_delete_files(b: &mut Bencher) {
	let dir = tempdir().unwrap();

	b.iter(|| {
		// For each iteration, create and then delete 100 files
		let files = setup_test_directory(dir.path(), 100);

		let found_files: Vec<FoundFile> = files.iter().map(|path| FoundFile::new(path)).collect();

		let (deleted_count, errors) = delete_files(&found_files);

		assert_eq!(deleted_count, 100);
		assert_eq!(errors.len(), 0);
	});
}

#[bench]
fn bench_find_and_delete(b: &mut Bencher) {
	let dir = tempdir().unwrap();

	b.iter(|| {
		// Set up fresh files for each iteration
		let _files = setup_test_directory(dir.path(), 100);

		// Find all .txt files
		let found_files = find_files(r"\.txt$", dir.path()).unwrap();

		// Should find about 1/4 of the files
		assert!(found_files.len() > 20);

		// Delete them
		let (deleted_count, errors) = delete_files(&found_files);

		assert_eq!(deleted_count, found_files.len());
		assert_eq!(errors.len(), 0);
	});
}

#[bench]
fn bench_nested_directories(b: &mut Bencher) {
	let root_dir = tempdir().unwrap();

	// Create a nested directory structure
	for i in 0..5 {
		let nested_dir = root_dir.path().join(format!("level_{}", i));
		std::fs::create_dir_all(&nested_dir).unwrap();

		// Add 20 files to each directory
		setup_test_directory(&nested_dir, 20);
	}

	b.iter(|| {
		// Find all .DS_Store files in the nested structure
		let found_files = find_files(r"\.DS_Store", root_dir.path()).unwrap();

		// Should find about 25 files (1/4 of 100 total files)
		assert!(found_files.len() > 20);
	});
}
