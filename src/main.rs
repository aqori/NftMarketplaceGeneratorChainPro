// src/main.rs
/*
 * Main executable for NftMarketplaceGeneratorChainPro
 */

use clap::Parser;
use nftmarketplacegeneratorchainpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceGeneratorChainPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
