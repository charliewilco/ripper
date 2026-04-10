use std::io::{self, Write};

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use ripper::{delete_files, find_files_with_options, SearchOptions};

/// Search for files by pattern and optionally delete them
#[derive(Parser)]
#[clap(name = "ripper", version, author, about)]
struct Cli {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Args, Clone, Debug)]
struct SearchArgs {
	/// Regex pattern to match filenames
	#[clap(required = true)]
	pattern: String,

	/// Directory to search in (defaults to current dir)
	#[clap(short, long, default_value = ".")]
	dir: String,

	/// Show verbose output
	#[clap(short, long)]
	verbose: bool,

	/// Follow symlinked directories while walking
	#[clap(long)]
	follow_links: bool,
}

#[derive(Args, Clone, Debug)]
struct DeleteArgs {
	#[clap(flatten)]
	search: SearchArgs,

	/// Automatically confirm deletion without prompting
	#[clap(short, long)]
	yes: bool,
}

#[derive(Subcommand)]
enum Commands {
	/// Find files matching a pattern without deleting them
	Find(SearchArgs),

	/// Delete files matching a pattern
	Delete(DeleteArgs),
}

fn main() -> Result<()> {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Find(args) => run_find(args),
		Commands::Delete(args) => run_delete(args),
	}
}

fn run_find(args: &SearchArgs) -> Result<()> {
	let file_list = find_matching_files(args)?;

	if file_list.is_empty() {
		println!("{}", "No matching files found.".yellow());
		return Ok(());
	}

	print_matches(&file_list);
	Ok(())
}

fn run_delete(args: &DeleteArgs) -> Result<()> {
	let file_list = find_matching_files(&args.search)?;

	if file_list.is_empty() {
		println!("{}", "No matching files found.".yellow());
		return Ok(());
	}

	print_matches(&file_list);

	let should_delete = if args.yes {
		true
	} else {
		print!("{} ", "Do you want to delete all these files? (y/n):".yellow().bold());
		io::stdout().flush()?;

		let mut response = String::new();
		io::stdin().read_line(&mut response)?;

		response.trim().eq_ignore_ascii_case("y")
	};

	if !should_delete {
		println!("{}", "Operation cancelled. No files were deleted.".yellow());
		return Ok(());
	}

	println!("{}", "Deleting files...".red().bold());

	let (deleted_count, errors) = delete_files(&file_list);

	println!(
		"{} {} {} {}",
		"Successfully deleted".green().bold(),
		deleted_count.to_string().green().bold(),
		"out of".green(),
		file_list.len().to_string().green()
	);

	if errors.is_empty() {
		return Ok(());
	}

	println!("{}", "Errors:".red().bold());
	for (path, error) in &errors {
		println!("  {} {}: {}", "Failed to delete".red(), path.display(), error);
	}

	Err(anyhow!("Failed to delete {} out of {} matching files", errors.len(), file_list.len()))
}

fn find_matching_files(args: &SearchArgs) -> Result<Vec<ripper::FoundFile>> {
	if args.verbose {
		println!("{} {}", "Searching for:".blue().bold(), args.pattern);
		println!("{} {}", "Starting from:".blue().bold(), args.dir);
		println!(
			"{} {}",
			"Following links:".blue().bold(),
			if args.follow_links { "yes" } else { "no" }
		);
	} else {
		println!("Searching...");
	}

	find_files_with_options(
		&args.pattern,
		&args.dir,
		SearchOptions { follow_links: args.follow_links },
	)
}

fn print_matches(file_list: &[ripper::FoundFile]) {
	println!("{} {}", "Found".green().bold(), file_list.len().to_string().green().bold());
	for (index, file) in file_list.iter().enumerate() {
		println!("[{}] {}", (index + 1).to_string().cyan(), file.path.display());
	}
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
