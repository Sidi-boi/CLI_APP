use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive:bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Missing arguements");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn read_file(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let mut res = Vec::new();

    if config.case_sensitive{
        res = search_case_sensitive(&config.query, &contents);
    } else {
       res =  search_case_insensitive(&config.query, &contents);
    }

    println!("the lines which contain '{}' are:", config.query);
    for element in res{
        println!("{}", element);
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut vec = Vec::new();

    for line in contents.lines(){
        if line.contains(&query){
            vec.push(line);
        }
    }

    vec
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }

    res
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive_test(){
        let query = "/";
        let contents = "/
sidi boi
hello world";

        assert_eq!(vec!["/"], search_case_sensitive(query, contents)); 
    }

    #[test]
    fn case_insensitive_test(){
        let query = "boi";
        let contents = "/
sidi boi
hello world
world boi";

        assert_eq!(vec!["sidi boi", "world boi"], search_case_sensitive(query, contents)); 
    }
}