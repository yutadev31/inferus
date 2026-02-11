use clap::{Parser, Subcommand};

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

fn run_format(target: Vec<String>, check_only: bool, verbose: bool) {
    if verbose {
        println!("Formatting files: {:?}", target.join(", "));
    }

    todo!()
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
