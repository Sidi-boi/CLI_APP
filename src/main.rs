use std::env;
use std::fs;
use std::process;

struct Config{
    query:String,
    filename:String,
}

impl Config{
    fn new(args:&[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("Missing arguements")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{ query, filename })
    }
}

fn read_file(config:Config){
    let contents = fs::read_to_string(config.filename).expect("Something went wrong while reading the file");
    
    println!("With text: {}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|error|{
        println!("Problem parsing arguments {}",  error);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}",  config.filename);
    
    read_file(config);
}
