// src/main.rs
/*
 * Main executable for UltraVisual
 */

use clap::Parser;
use ultravisual::{Result, run};

#[derive(Parser)]
#[command(version, about = "UltraVisual - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
