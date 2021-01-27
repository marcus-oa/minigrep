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

// As part of 'Separation of concerns for Binary Projects
// we remove all code from main which doesn't involve setting
// up the config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!{"{}", line};
    }

    Ok(())
}

// Note: remember, we want lifetimes to param values we return
// and as we only ever return references to contents we don't need
// to apply to the 'query' param
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // convenient method for line iteration (.lines())
    for line in contents.lines() {
        // again, another convenient method for checking the
        // contents of a string
        if line.contains(query) {
            // store the line in a vector
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}