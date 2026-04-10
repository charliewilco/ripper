use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use ripper::{delete_files, find_files, FoundFile};
use tempfile::{tempdir, TempDir};

fn setup_test_directory(dir_path: &Path, num_files: usize) -> Vec<PathBuf> {
	let mut created_files = Vec::with_capacity(num_files);

	for index in 0..num_files {
		let file_path = match index % 4 {
			0 => dir_path.join(format!("file_{index}.txt")),
			1 => dir_path.join(format!("file_{index}.log")),
			2 => dir_path.join(format!(".DS_Store_{index}")),
			_ => dir_path.join(format!("file_{index}.cfg")),
		};

		File::create(&file_path).unwrap().write_all(b"benchmark test content").unwrap();
		created_files.push(file_path);
	}

	created_files
}

fn found_files_for_delete(num_files: usize) -> (TempDir, Vec<FoundFile>) {
	let dir = tempdir().unwrap();
	let files = setup_test_directory(dir.path(), num_files);
	let found_files = files.iter().map(FoundFile::new).collect();
	(dir, found_files)
}

fn benchmark_find_files_small(c: &mut Criterion) {
	let dir = tempdir().unwrap();
	let _files = setup_test_directory(dir.path(), 50);

	c.bench_function("find_files_small", |bench| {
		bench.iter(|| {
			let results = find_files(r"\.txt$", dir.path()).unwrap();
			black_box(results.len());
		});
	});
}

fn benchmark_find_files_medium(c: &mut Criterion) {
	let dir = tempdir().unwrap();
	let _files = setup_test_directory(dir.path(), 500);

	c.bench_function("find_files_medium", |bench| {
		bench.iter(|| {
			let results = find_files(r"\.txt$", dir.path()).unwrap();
			black_box(results.len());
		});
	});
}

fn benchmark_find_complex_pattern(c: &mut Criterion) {
	let dir = tempdir().unwrap();
	let _files = setup_test_directory(dir.path(), 200);

	c.bench_function("find_complex_pattern", |bench| {
		bench.iter(|| {
			let results = find_files(r"\.txt$|\.log$|\.DS_Store", dir.path()).unwrap();
			black_box(results.len());
		});
	});
}

fn benchmark_delete_files(c: &mut Criterion) {
	c.bench_function("delete_files", |bench| {
		bench.iter_batched(
			|| found_files_for_delete(100),
			|(_dir, found_files)| {
				let (deleted_count, errors) = delete_files(&found_files);
				assert_eq!(deleted_count, 100);
				assert_eq!(errors.len(), 0);
			},
			BatchSize::SmallInput,
		);
	});
}

fn benchmark_find_and_delete(c: &mut Criterion) {
	c.bench_function("find_and_delete", |bench| {
		bench.iter_batched(
			|| {
				let dir = tempdir().unwrap();
				let _files = setup_test_directory(dir.path(), 100);
				dir
			},
			|dir| {
				let found_files = find_files(r"\.txt$", dir.path()).unwrap();
				assert!(found_files.len() > 20);

				let (deleted_count, errors) = delete_files(&found_files);
				assert_eq!(deleted_count, found_files.len());
				assert_eq!(errors.len(), 0);
			},
			BatchSize::SmallInput,
		);
	});
}

fn benchmark_nested_directories(c: &mut Criterion) {
	let root_dir = tempdir().unwrap();

	for index in 0..5 {
		let nested_dir = root_dir.path().join(format!("level_{index}"));
		std::fs::create_dir_all(&nested_dir).unwrap();
		setup_test_directory(&nested_dir, 20);
	}

	c.bench_function("nested_directories", |bench| {
		bench.iter(|| {
			let found_files = find_files(r"\.DS_Store", root_dir.path()).unwrap();
			black_box(found_files.len());
		});
	});
}

criterion_group!(
	benches,
	benchmark_find_files_small,
	benchmark_find_files_medium,
	benchmark_find_complex_pattern,
	benchmark_delete_files,
	benchmark_find_and_delete,
	benchmark_nested_directories
);
criterion_main!(benches);
