use std::{env, process};
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config:Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = mini_grep::run(config) {
        println!("Application error : {}", e);
        process::exit(1);
    };

}


