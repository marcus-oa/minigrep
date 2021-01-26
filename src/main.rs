// Bringing in std::env means the calls for env
// functions are less ambiguous (i.e. env::args is from env)
use std::env;
use std::fs;

fn main() {
    // Collect can create many kinds of collections
    // Specifying the type of the variable ensures we
    // create said type
    let args: Vec<String> = env::args().collect();

    // Note: &args[0] = `target/debug/minigrep`
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
