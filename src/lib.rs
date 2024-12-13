use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub search_phrase: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() != 3 {
            return Err("Please provide two arguments, filename and search string.".into());
        }

        let search_phrase = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            search_phrase,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.search_phrase);
    println!("In file {}", config.filename);

    let content = read_lines(&config.filename);
    let results = if config.ignore_case {
        search_case_insensitive(&config.search_phrase, &content)
    } else {
        search(&config.search_phrase, &content)
    };

    if results.is_empty() {
        return Err("No results found.".into());
    }

    println!("{:#?}", results);
    Ok(())
}

pub fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn search<'a>(query: &str, content: &'a [String]) -> Vec<String> {
    let mut line_number = 0;
    let mut results: Vec<String> = Vec::new();

    for line in content {
        line_number += 1;
        if line.contains(&query) {
            results.push(format!(
                "Found occurence of {} on line number {}: {}",
                query, line_number, line
            ));
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a [String]) -> Vec<String> {
    let mut line_number = 0;
    let mut results: Vec<String> = Vec::new();
    let query = query.to_lowercase();

    for line in content {
        line_number += 1;
        if line.to_lowercase().contains(&query) {
            results.push(format!(
                "Found occurence of {} on line number {}: {}",
                query, line_number, line
            ));
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
        let contents = vec![
            String::from("Rust: safe, fast, productive."),
            String::from("Pick three."),
            String::from("Trust me."),
            String::from("Duckt tape."),
        ];

        assert_eq!(
            vec!["Found occurence of duct on line number 1: Rust: safe, fast, productive."],
            search(query, &contents)
        );
    }

    #[test]
    fn test_insensitive() {
        let query = "rUst";
        let contents = vec![
            String::from("Rust: safe, fast, productive."),
            String::from("Pick three."),
            String::from("Trust me."),
        ];
        assert_eq!(
            search_case_insensitive(query, &contents),
            vec![
                "Found occurence of rust on line number 1: Rust: safe, fast, productive.",
                "Found occurence of rust on line number 3: Trust me."
            ]
        );
    }
}
