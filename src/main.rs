// use std::fs;
use std::{env, process};
use lan_file_share::{Config, run};

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("An error occurred while trying to parse the arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1)
    }

}
