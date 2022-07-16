use std::env;
use std::error::Error;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {} in {}", config.query, config.filename);

    let contents: String = fs::read_to_string(config.filename)?;
    println!("Retrieved contents: \n {}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // We will modify this to use lifetimes instead of cloning later on.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
        })
    }
}
