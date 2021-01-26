use std::error::Error;
use std::fs;

// Necessary abstraction
pub struct Config {
    pub query: String,
    pub filename: String
}

// Idiomatic implementation of Config
impl Config {
    // 'static lifetime ensures the str value is stored for the
    // entire lifetime of the program (Useful for errors)
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguements");
        }
        // Note: &args[0] = `target/debug/minigrep`
        // Note+: Clone ensures that we don't violate rust rules
        // of taking ownership of the values passed into the fn
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// As part of 'Seperation o concerns for Binary Projects
// we remove all code from main which doesn't involve setting
// up the config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}