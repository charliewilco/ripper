use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use grep_regex::RegexMatcher;
use walkdir::WalkDir;

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
        Commands::Find {
            pattern,
            dir,
            yes,
            verbose,
        } => find_and_delete(pattern, dir, *yes, *verbose)?,
    }

    Ok(())
}

fn find_and_delete(pattern: &str, start_dir: &str, auto_confirm: bool, verbose: bool) -> Result<()> {
    if verbose {
        println!("{} {}", "Searching for:".blue().bold(), pattern);
        println!("{} {}", "Starting from:".blue().bold(), start_dir);
    } else {
        println!("Searching...");
    }
    
    // Create a regex matcher for the file name
    let matcher = RegexMatcher::new(pattern)
        .context("Invalid regex pattern")?;
    
    // Find matching files
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
                    file_list.push(path.to_path_buf());
                }
            }
        }
    }
    
    if file_list.is_empty() {
        println!("{}", "No matching files found.".yellow());
        return Ok(());
    }
    
    // Display found files
    println!("{} {}", "Found".green().bold(), file_list.len().to_string().green().bold());
    for (i, file) in file_list.iter().enumerate() {
        println!("[{}] {}", (i + 1).to_string().cyan(), file.display());
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
        
        let mut deleted_count = 0;
        let mut errors = Vec::new();
        
        for file in &file_list {
            match fs::remove_file(file) {
                Ok(_) => {
                    if verbose {
                        println!("{} {}", "Deleted:".green(), file.display());
                    }
                    deleted_count += 1;
                },
                Err(e) => {
                    let error_msg = format!("Failed to delete {}: {}", file.display(), e);
                    errors.push(error_msg);
                },
            }
        }
        
        println!(
            "{} {} {} {}",
            "Successfully deleted".green().bold(),
            deleted_count.to_string().green().bold(),
            "out of".green(),
            file_list.len().to_string().green()
        );
        
        if !errors.is_empty() {
            println!("{}", "Errors:".red().bold());
            for error in errors {
                println!("  {}", error.red());
            }
        }
    } else {
        println!("{}", "Operation cancelled. No files were deleted.".yellow());
    }
    
    Ok(())
}
