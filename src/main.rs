use std::env;
use std::process;

use fgrep::Config;

fn main() {
    // here the error handled using unwrap to collect the result
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // here if let is used since we only need to worry about error case
    if let Err(e) = fgrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
