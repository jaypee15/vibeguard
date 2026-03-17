mod scanner;
mod parser;
mod analyzer;

use std::fs;
use clap::{Parser, Subcommand};
use anyhow::Result;
use scanner::get_files_to_scan;
use parser::parse_javascript;
use analyzer::{analyze_javascript, Issue};

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
                if let Some(ext) = file.extension() {
                    if ext == "js" {
                        println!("Scanning file: {}", file.display());
                        
                        match fs::read_to_string(&file) {
                            Ok(content) => {
                                if let Some(tree) = parse_javascript(&content) {
                                    let issues = analyze_javascript(&content, &tree, &file);
                                    
                                    if !issues.is_empty() {
                                        println!("\n🚨 Found {} issues in {}", issues.len(), file.display());
                                        for issue in issues {
                                            println!("[Line {}] {}: {}", issue.line, issue.rule_id, issue.message);
                                        }
                                    } else {
                                        println!("No issues found in file: {}", file.display());
                                    }
                                } else {
                                    println!("Failed to parse file: {}", file.display());
                                }
                            } 
                            Err(e) => {
                                println!("Failed to read file: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}