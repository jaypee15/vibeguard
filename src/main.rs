mod analyzer;
mod parser;
mod rule_engine;
mod scanner;
mod mcp;

use analyzer::{Issue, analyze_javascript};
use anyhow::Result;
use clap::{Parser, Subcommand};
use parser::parse_javascript;
use rayon::prelude::*;
use rule_engine::load_rules;
use scanner::get_files_to_scan;
use std::fs;
use serde_json;
use mcp::run_mcp_server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        #[arg(default_value = ".")]
        path: String,

        #[arg(short, long)]
        json: bool,
    },
    Mcp,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { path, json } => {
            println!("Vibeguard is preparing to scan...");
            println!("Target path: {}", path);

            let rules = load_rules("rules/javascript.yaml");
            println!("Loaded {} rules.", rules.len());

            let files = get_files_to_scan(path);
            println!("Found {} files to scan!", files.len());

            let start_time = std::time::Instant::now();

            let all_issues: Vec<Issue> = files
                .par_iter()
                .flat_map(|file| {
                    let mut file_issues = Vec::new();

                    if let Some(ext) = file.extension() {
                        if ext == "js" {
                            if let Ok(content) = fs::read_to_string(file) {
                                if let Some(tree) = parse_javascript(&content) {
                                    file_issues = analyze_javascript(&content, &tree, file, &rules);
                                }
                            }
                        }
                    }
                    file_issues
                })
                .collect();

            let duration = start_time.elapsed();
            if *json{
                let json_output = serde_json::to_string_pretty(&all_issues).expect("Failed to serialize to JSON");
                println!("{}", json_output);
            } else {
                println!("\n=== SCAN COMPLETE IN {:?} ===", duration);
                
                if all_issues.is_empty() {
                    println!("✅No vulnerabilities found.");
                } else {
                    println!("\n🚨 Found {} total issues:\n", all_issues.len());
    
                    for issue in all_issues {
                        println!(
                            "[{}] {} (Line {}) {}",
                            issue.severity,
                            issue.file.display(),
                            issue.line,
                            issue.message
                        );
                    }
                }
            }

           
        }
        Commands::Mcp => {
            run_mcp_server();
        }
    }

    Ok(())
}
