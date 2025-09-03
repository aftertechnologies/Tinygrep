use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in file {}", config.file_path);

    if let Err(_e) = minigrep::run(config) {
        eprintln!("Application Error");
        process::exit(1);
    }
}
