mod analyzer;
mod mcp;
mod parser;
mod rule_engine;
mod scanner;

use anyhow::Result;
use clap::{Parser, Subcommand};
use mcp::run_mcp_server;
use scanner::run_scan;

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
            let start_time = std::time::Instant::now();

            let all_issues = run_scan(path);

            let duration = start_time.elapsed();
            if *json {
                let json_output =
                    serde_json::to_string_pretty(&all_issues).expect("Failed to serialize to JSON");
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
