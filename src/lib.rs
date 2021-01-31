use std::error::Error;
use std::{fs, env};
use std::env::Args;

// Necessary abstraction
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// Idiomatic implementation of Config
impl Config {
    // 'static lifetime ensures the str value is stored for the
    // entire lifetime of the program (Useful for errors)
    // Note: 'new' function now accepts env:Args as a mutable owned
    // iterator
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // required to get passed the name of the program which is the default first
        // argument
        args.next();

        // Clone usage replaced with direct iteration over args
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // Note: Checks whether CASE_INSENSITIVE=1 is set
        // if not, is_err() is true and the value is set to true, otherwise false
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// As part of 'Separation of concerns for Binary Projects
// we remove all code from main which doesn't involve setting
// up the config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!{"{}", line};
    }

    Ok(())
}

// Note: remember, we want lifetimes to param values we return
// and as we only ever return references to contents we don't need
// to apply to the 'query' param
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// An implementation of search_case_sensitive but for insensitive values
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}