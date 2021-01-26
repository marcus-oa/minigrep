// Bringing in std::env means the calls for env
// functions are less ambiguous (i.e. env::args is from env)
use std::env;
use std::fs;

fn main() {
    // Collect can create many kinds of collections
    // Specifying the type of the variable ensures we
    // create said type
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// Necessary abstraction
struct Config {
    query: String,
    filename: String
}

// Note: As a rule we keep the argument passing from command
// line within the main.rs file as long as it relatively short
fn parse_config(args: &[String]) -> Config {
    // Note: &args[0] = `target/debug/minigrep`
    // Note+: Clone ensures that we don't violate rust rules
    // of taking ownership of the values passed into the fn
    let query = args[1].clone;
    let filename = args[2].clone;

    Config { query, filename }
}
