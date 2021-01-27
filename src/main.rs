// Bringing in std::env means the calls for env
// functions are less ambiguous (i.e. env::args is from env)
use std::{env, process};
use minigrep::Config;

fn main() {
    // Collect can create many kinds of collections
    // Specifying the type of the variable ensures we
    // create said type
    let args: Vec<String> = env::args().collect();

    // Handling errors here produces a nicer output to the terminal
    // on arg pass failure
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    // Note: we use 'if let' instead of unwrap_or_else as
    // run(config) doesn't return a value we want to unwrap
    // unlike config above
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}