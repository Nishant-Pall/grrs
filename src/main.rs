use std::env;
use std::process;

use grrs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grrs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
