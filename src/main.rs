#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
/// Search for a pattern in a file and display the lines that contain it.
struct Cli {
    // The pattern to look for
    pattern: String,

    // The path to file to be read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // The parse method is meant to be used in your main function.
    // When it fails, it will print out an error or help message and
    // immediately exit the program. Don’t use it in other places!

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
