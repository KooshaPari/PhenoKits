/// target-pruner: Proactive Rust target/ directory cleanup tool.
///
/// Scans repos for target/ directories >500MB and offers:
/// - --dry-run: report sizes without deleting
/// - --prune: delete all oversized targets
/// - --threshold: custom size threshold (bytes)
///
/// Justification: Rust for path traversal + size calc is tighter than sh/bash,
/// provides --dry-run safety, and integrates with disk-budget governance.

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    #[value(name = "dry-run")]
    DryRun,
    #[value(name = "prune")]
    Prune,
}

#[derive(Parser, Debug)]
#[command(
    name = "target-pruner",
    about = "Find and optionally delete oversized Rust target/ directories"
)]
struct Args {
    /// Repository root or search path
    #[arg(default_value = ".", value_name = "PATH")]
    path: PathBuf,

    /// Size threshold in MB (default: 500)
    #[arg(short, long, default_value = "500")]
    threshold: u64,

    /// Operation mode
    #[arg(value_enum, default_value = "dry-run")]
    mode: Mode,

    /// Search depth (default: 3 levels)
    #[arg(short, long, default_value = "3")]
    depth: usize,

    /// Verbose output
    #[arg(short)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let threshold_bytes = args.threshold * 1024 * 1024;

    if !args.path.exists() {
        return Err(anyhow!("Path not found: {}", args.path.display()));
    }

    let mut candidates = Vec::new();

    // Recursively find target/ directories
    for entry in WalkDir::new(&args.path)
        .max_depth(args.depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "target" && e.file_type().is_dir())
    {
        let size = calculate_dir_size(entry.path())?;

        if size > threshold_bytes {
            candidates.push((entry.path().to_path_buf(), size));
        }
    }

    if candidates.is_empty() {
        println!(
            "No target/ directories >{}MB found.",
            args.threshold
        );
        return Ok(());
    }

    // Report findings
    println!(
        "Found {} oversized target/ directories:\n",
        candidates.len()
    );

    let mut total_size = 0u64;
    for (path, size) in &candidates {
        let size_mb = size / (1024 * 1024);
        println!("  {} ({}MB)", path.display(), size_mb);
        total_size += size;

        if args.verbose {
            if let Ok(entries) = fs::read_dir(path) {
                let count = entries.count();
                println!("    └─ {} entries", count);
            }
        }
    }

    println!("\nTotal: {}MB", total_size / (1024 * 1024));

    match args.mode {
        Mode::DryRun => {
            println!("\n(Dry-run mode: no changes made)");
            println!("Run with '--mode prune' to delete.");
            Ok(())
        }
        Mode::Prune => {
            println!("\nRemoving {} directories...", candidates.len());
            let mut deleted = 0;

            for (path, size) in candidates {
                match fs::remove_dir_all(&path) {
                    Ok(_) => {
                        let size_mb = size / (1024 * 1024);
                        println!("  Deleted: {} ({}MB)", path.display(), size_mb);
                        deleted += 1;
                    }
                    Err(e) => {
                        eprintln!("  Error deleting {}: {}", path.display(), e);
                    }
                }
            }

            println!("\nReclaimed: {}/{} directories", deleted, candidates.len());
            Ok(())
        }
    }
}

/// Recursively calculate directory size in bytes.
fn calculate_dir_size(path: &Path) -> Result<u64> {
    let mut total = 0u64;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total += metadata.len();
            }
        }
    }

    Ok(total)
}
