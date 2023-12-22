
use clap::Parser;

/* Search for a pattern in a file and display the lines */
#[derive(Parser)]
struct Cli {
    // Pattern to look for
    pattern: String,
    // Path to the file
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}