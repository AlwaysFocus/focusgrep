use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {} in {}", query, filename);
    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong trying to read the file...");
    println!("Retrieved contents: \n {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // We will modify this to use lifetimes instead of cloning later on.
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {
        query: query.to_string(),
        filename: filename.to_string(),
    }
}
