mod analyzer;
mod mcp;
mod parser;
mod rule_engine;
mod scanner;
mod taint;

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
    Install,
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
                    println!("✅ No vulnerabilities found!");
                } else {
                    println!("🚨 Found {} total issues:\n", all_issues.len());
                    for issue in &all_issues {
                        println!(
                            "[{}] {} (Line {}): {}",
                            issue.severity.to_uppercase(),
                            issue.file.display(),
                            issue.line,
                            issue.message
                        );
                    }
                }
            }

            // EXIT CODE LOGIC: If we found issues, exit with code 1 so Git blocks the commit!
            if !all_issues.is_empty() {
                std::process::exit(1);
            }
        }

        Commands::Mcp => {
            run_mcp_server();
        }

        // NEW INSTALL COMMAND
        Commands::Install => {
            let git_dir = std::path::Path::new(".git");
            if !git_dir.exists() {
                eprintln!(
                    "❌ Error: Not a git repository. Please run this in the root of a project with a .git folder."
                );
                std::process::exit(1);
            }

            let hook_path = git_dir.join("hooks").join("pre-commit");

            // The Bash script that Git will execute before a commit
            let hook_script = r#"#!/bin/sh
    echo "🛡️  Running VibeGuard security scan..."
    
    # Run the globally installed vibeguard command
    vibeguard scan .
    
    # $? checks the exit code of the previous command
    if [ $? -ne 0 ]; then
        echo ""
        echo "❌ VibeGuard found vulnerabilities! Commit blocked."
        echo "Please fix the issues above or remove the insecure code."
        exit 1
    fi
    
    echo "✅ VibeGuard scan passed! Committing..."
    "#;

            // Write the file to .git/hooks/pre-commit
            std::fs::write(&hook_path, hook_script).expect("Failed to write pre-commit hook");

            // Make the script executable (chmod +x) - but ONLY on Mac/Linux!
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&hook_path).unwrap().permissions();
                perms.set_mode(0o755); // 755 means rwxr-xr-x
                std::fs::set_permissions(&hook_path, perms).unwrap();
            }

            println!(
                "✅ VibeGuard pre-commit hook installed successfully at .git/hooks/pre-commit"
            );
        }
    }

    Ok(())
}
