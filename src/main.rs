use std::io::{self, Write};

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use ripper::{delete_files, find_files};

/// Search for files by pattern and optionally delete them
#[derive(Parser)]
#[clap(name = "ripper", version, author, about)]
struct Cli {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Find files matching a pattern
	Find {
		/// Regex pattern to match filenames
		#[clap(required = true)]
		pattern: String,

		/// Directory to search in (defaults to current dir)
		#[clap(short, long, default_value = ".")]
		dir: String,

		/// Automatically confirm deletion without prompting
		#[clap(short, long)]
		yes: bool,

		/// Show verbose output
		#[clap(short, long)]
		verbose: bool,
	},
}

fn main() -> Result<()> {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Find { pattern, dir, yes, verbose } => {
			find_and_delete(pattern, dir, *yes, *verbose)?
		}
	}

	Ok(())
}

fn find_and_delete(
	pattern: &str,
	start_dir: &str,
	auto_confirm: bool,
	verbose: bool,
) -> Result<()> {
	if verbose {
		println!("{} {}", "Searching for:".blue().bold(), pattern);
		println!("{} {}", "Starting from:".blue().bold(), start_dir);
	} else {
		println!("Searching...");
	}

	// Find matching files
	let file_list = find_files(pattern, start_dir)?;

	if file_list.is_empty() {
		println!("{}", "No matching files found.".yellow());
		return Ok(());
	}

	// Display found files
	println!("{} {}", "Found".green().bold(), file_list.len().to_string().green().bold());
	for (i, file) in file_list.iter().enumerate() {
		println!("[{}] {}", (i + 1).to_string().cyan(), file.path.display());
	}

	// Ask if user wants to delete the files
	let should_delete = if auto_confirm {
		true
	} else {
		print!("{} ", "Do you want to delete all these files? (y/n):".yellow().bold());
		io::stdout().flush()?;

		let mut response = String::new();
		io::stdin().read_line(&mut response)?;

		response.trim().to_lowercase() == "y"
	};

	if should_delete {
		println!("{}", "Deleting files...".red().bold());

		let (deleted_count, errors) = delete_files(&file_list);

		println!(
			"{} {} {} {}",
			"Successfully deleted".green().bold(),
			deleted_count.to_string().green().bold(),
			"out of".green(),
			file_list.len().to_string().green()
		);

		if !errors.is_empty() {
			println!("{}", "Errors:".red().bold());
			for (path, error) in errors {
				println!("  {} {}: {}", "Failed to delete".red(), path.display(), error);
			}
		}
	} else {
		println!("{}", "Operation cancelled. No files were deleted.".yellow());
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cli_parsing() {
		use clap::CommandFactory;
		Cli::command().debug_assert();
	}
}
