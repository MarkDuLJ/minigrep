use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}



impl Config {
    // change args type from &[String] to impl Iterator<Item = String>
    // now args can be any type that implements the Iterator trait and returns String items.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        
        // remove clone from borrowed string to iter using next()
        
        args.next();
        let query =match args.next() {
            Some(arg) => arg,
            None => return Err("No query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path"),
        };
        
        // if args.len() < 3 {
        //     return Err("need two arguments");
        // }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        for (key, value) in env::vars_os() {
            println!("{:?}: {:?}", key, value);
        }
     
        if ignore_case {
            println!("Ignoring case for comparisons.");
        } else {
            println!("Case-sensitive comparisons.");
        }

        Ok(Config { query, file_path, ignore_case})
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query,&content)
    }else {
        search(&config.query, &content)
    };
    
    for line in result{
        println!("{line}");
    }
    Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // remove for loop, using iterator
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
   /*  let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
    */
}

pub fn search_case_insensitive<'a>(
        query: &str, 
        contents: &'a str
    ) -> Vec<&'a str>{

        let query = query.to_lowercase();
        let mut result = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query){
                result.push(line)
            }
        }

        result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {

        let query = "duct"; 
        let contents = "\
    Rust:
safe, fast, productive.
    Pick three.
    Duct tape.";
    
            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }


    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
