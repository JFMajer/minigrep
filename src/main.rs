use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    search_phrase: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Please provide two arguments, search phrase and filename.");
        }

        let search_phrase = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {
            search_phrase,
            filename,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.search_phrase);
    println!("In file {}", config.filename);

    let mut line_number = 0;
    let mut results: Vec<String> = Vec::new();

    let content = read_lines(&config.filename);
    for line in content {
        line_number += 1;
        if line.contains(&config.search_phrase) {
            results.push(format!(
                "Found occurence of {} on line number {}: {}",
                config.search_phrase, line_number, line
            ));
        }
    }

    if results.is_empty() {
        return Err("No results found.".into());
    }

    println!("{:#?}", results);
    Ok(())
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
