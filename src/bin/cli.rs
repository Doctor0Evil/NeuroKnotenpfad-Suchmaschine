//! Command-line interface for CADSP
//! Provides interactive scanning, pattern detection, and analysis

use std::io::{self, Write};
use cadsp_core::*;

#[tokio::main]
async fn main() {
    println!();
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   🧠 CADSP: Cybernetic AI Design Synthesis Platform       ║");
    println!("║      NeuroNodePath Engine v0.1                            ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    let github_token = std::env::var("GITHUB_TOKEN")
        .unwrap_or_else(|_| {
            eprintln!("⚠️  GITHUB_TOKEN not set. Repository scanning disabled.");
            String::new()
        });

    let scanner = RepositoryScanner::new(github_token);

    loop {
        println!();
        println!("Commands:");
        println!("  scan <url>     - Scan a GitHub repository");
        println!("  analyze <code> - Analyze code for biophysical patterns");
        println!("  help           - Show this help");
        println!("  exit           - Exit CADSP");
        println!();
        print!("cadsp> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "exit" => {
                println!("Goodbye! 🧠");
                return;
            }
            "help" => {
                println!("CADSP Help");
                println!("  scan <url>  - Scan GitHub repository at <url>");
                println!("  analyze     - Analyze code patterns");
            }
            s if s.starts_with("scan ") => {
                let url = s.trim_start_matches("scan ").trim();
                println!("📡 Scanning: {}", url);

                match scanner.scan(url).await {
                    Ok(metadata) => {
                        println!("✓ Scan completed!");
                        println!("  Scan ID: {}", metadata.scan_id);
                        println!("  Primary language: {}", metadata.metadata.primary_language);
                        println!("  File count: {}", metadata.metadata.file_count);
                        println!("  Languages: {:?}", metadata.metadata.secondary_languages);
                    }
                    Err(e) => println!("✗ Scan failed: {}", e),
                }
            }
            _ => {
                println!("Unknown command. Try 'help' or 'exit'.");
            }
        }
    }
}
