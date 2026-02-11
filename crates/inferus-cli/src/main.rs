use std::fs;

use anyhow::Context;
use clap::{Parser, Subcommand};
use inferus_format::format_markdown;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Format {
        #[arg()]
        target: Vec<String>,

        #[arg(short, long, default_value_t = false)]
        check_only: bool,

        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
    Lint {
        #[arg()]
        target: Vec<String>,

        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

fn get_file_list(target: Vec<String>) -> Vec<String> {
    let mut files = Vec::new();

    for item in target {
        if !item.ends_with(".md") {
            println!("Warning: Skipping non-markdown file: {}", item);
            continue;
        }
        files.push(item);
    }

    if files.is_empty() {
        eprintln!("No markdown files found");
        std::process::exit(1);
    }

    files
}

fn format_file(path: &str) -> anyhow::Result<()> {
    let text = fs::read_to_string(path).context(format!("Failed to read file: {}", path))?;
    let formated = format_markdown(&text);
    fs::write(path, formated).context(format!("Failed to write file: {}", path))?;

    Ok(())
}

fn run_format(target: Vec<String>, check_only: bool, verbose: bool) {
    if verbose {
        println!("Formatting files: {}", target.join(", "));
    }

    let files = get_file_list(target);

    for path in files {
        println!("test0");
        match format_file(&path) {
            Ok(()) => println!("Formatted file: {}", path),
            Err(e) => eprintln!("Error formatting file {}: {}", path, e),
        }
    }
}

fn run_lint(target: Vec<String>, verbose: bool) {
    if verbose {
        println!("Linting files: {:?}", target.join(", "));
    }

    todo!()
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Format {
            target,
            check_only,
            verbose,
        } => run_format(target, check_only, verbose),
        Command::Lint { target, verbose } => run_lint(target, verbose),
    }
}
