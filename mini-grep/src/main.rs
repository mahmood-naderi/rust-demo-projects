use std::{env, fs, process, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config:Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error : {}", e);
        process::exit(1);
    };

}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("{}", content);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments...");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
    
        Ok(Config { query, filename })
    }
}


