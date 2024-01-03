use std::env;
use std::process;

use cli_app::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|error|{
        println!("Problem parsing arguments {}",  error);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}",  config.filename);
    
    if let Err(e) = cli_app::read_file(config){
        println!("Error Occurred: {}", e);
        process::exit(1);
    }
}






