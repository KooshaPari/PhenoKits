//! Phenotype Forge - CLI Task Runner
//!
//! A high-performance task runner with dependency resolution,
//! parallel execution, and hot reload support.

mod config;

use clap::Parser;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(name = "forge")]
#[command(about = "A high-performance task runner")]
struct Args {
    /// Task to run
    #[arg(default_value = "test")]
    task: String,

    /// Enable watch mode
    #[arg(short, long)]
    watch: bool,

    /// Configuration file path
    #[arg(short, long, default_value = "forge.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // Load and validate configuration
    let config_path = std::path::Path::new(&args.config);
    info!("Loading configuration from {:?}", config_path);

    let config = match config::load_and_validate(config_path) {
        Ok(cfg) => {
            info!(
                "Configuration loaded successfully: {} v{}",
                cfg.name, cfg.version
            );
            cfg
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e);
        }
    };

    // Execute task
    info!("Running task: {}", args.task);
    println!("Running task: {}", args.task);

    // Show available tasks
    println!("\nAvailable tasks in {}:", config.name);
    for task in &config.tasks {
        let desc = task.description.as_deref().unwrap_or(&task.command);
        if task.deps.is_empty() {
            println!("  - {}: {}", task.name, desc);
        } else {
            println!(
                "  - {} (deps: {}): {}",
                task.name,
                task.deps.join(", "),
                desc
            );
        }
    }

    if args.watch {
        println!("\nWatching for changes...");
    }

    Ok(())
}
