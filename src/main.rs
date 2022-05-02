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

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The parse method is meant to be used in your main function.
    // When it fails, it will print out an error or help message and
    // immediately exit the program. Don’t use it in other places!

    // one line solution for exiting if error exists
    // let result = std::fs::read_to_string(&args.path).unwrap();

    // let result = std::fs::read_to_string(&args.path);

    let args = Cli::parse();
    // You can append this operator to a value of type Result,
    // and Rust will internally expand this to something very similar to the match we just wrote.

    let path = "test.txt";

    let result = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading: `{}`: {}", path, err)));
    // let content = match result {
    //     Ok(content) => content,
    //     Err(error) => return Err(error.into()),
    // };
    println!("file content: {:?}", result);
    Ok(())
}
