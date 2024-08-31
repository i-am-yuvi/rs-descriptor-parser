use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short = 't', long = "text")]
    text: Option<String>,

    #[arg(short = 'p', long = "path")]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    match (args.text, args.path) {
        (Some(text), None) => {
            process_text(text);
        }
        (None, Some(path)) => {
            process_path(path);
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Please provide either text or path, not both.");
        }
        (None, None) => {
            eprintln!("Error: Please provide either --text (-t) or --path (-p) option.");
        }
    }
}

fn process_text(text: String) {}

fn process_path(path: String) {}
