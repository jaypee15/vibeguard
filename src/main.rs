mod scanner;

use clap::{Parser, Subcommand};
use anyhow::Result;
use scanner::get_files_to_scan;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Scan {
        #[arg(default_value = ".")]
        path: String,
        
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Scan { path, json } => {
            println!("Vibeguard is preparing to scan...");
            println!("Target path: {}", path);
            
            let files = get_files_to_scan(path);
            
            println!("Found {} files to scan!", files.len());
            
            for file in files.into_iter().take(5) {
                println!("Scanning: {}", file.display());
            }
        }
    }
    
    Ok(())
}