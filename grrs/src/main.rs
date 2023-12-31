

use std::io::BufRead;

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

    let file = std::fs::File::open(&args.path).unwrap();
    let reader = std::io::BufReader::new(file);
    

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}