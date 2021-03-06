use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(conf) => conf,
        Err(msg) => {
            println!("Problem parsing arguments: {}", msg);
            process::exit(1);
        }
    };
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
