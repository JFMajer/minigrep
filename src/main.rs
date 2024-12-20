use std::env;
use std::process;

use minigrep::{run, Config};

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
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
