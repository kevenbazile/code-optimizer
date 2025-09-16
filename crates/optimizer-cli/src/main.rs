use clap::Parser;
use std::fs;
use std::path::PathBuf;

use code_optimizer_core::{CodeOptimizer, Language};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    // path to code file
    #[arg(required = true)]
    file_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let code_content = match fs::read_to_string(&cli.file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "Error: failed to read file '{}': {}",
                cli.file_path.display(),
                e
            );
            std::process::exit(1);
        }
    };
    let language = match detect_language_from_path(&cli.file_path) {
        Some(lang) => lang,
        None => {
            eprintln!(
                "Error: Could not determine the programming language from the file extension."
            );
            eprintln!("Supported extensions are: .js, .ts, .py, .rs");
            std::process::exit(1);
        }
    };
    let optimizer = CodeOptimizer::new();
    let optimizations = optimizer.analyze_code(&code_content, language);

    if optimizations.is_empty() {
        println!(
            "✅ No optimization suggestions found for '{}'.",
            cli.file_path.display()
        );
    } else {
        println!(
            "🔍 Found {} potential optimizations in '{}':\n",
            optimizations.len(),
            cli.file_path.display()
        );
        for opt in optimizations {
            println!("--------------------------------------------------");
            println!(
                "🎯 Rule: '{}' (Confidence: {:.0}%)",
                opt.rule_name,
                opt.confidence * 100.0
            );
            println!("💡 Suggestion: {}", opt.explanation);
            println!("📍 Location: Line {}", opt.line_number);
            println!("   Original:   {}", opt.original_code.trim());
            println!("   Suggested:  {}", opt.suggested_code.trim());
            println!();
        }
        println!("--------------------------------------------------");
    }
}

fn detect_language_from_path(path: &PathBuf) -> Option<Language> {
    let extension = path.extension()?.to_str()?;

    match extension {
        "js" | "ts" => Some(Language::JavaScript),
        "py" => Some(Language::Python),
        "rs" => Some(Language::Rust),
        _ => None,
    }
}
