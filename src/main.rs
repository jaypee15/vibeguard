use clap::{Parser, Subcommand};
use anyhow::Result;

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
            println!("JSON output: {}", json);
        }
    }
    
    Ok(())
}