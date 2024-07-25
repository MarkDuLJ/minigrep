use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    // remove below line, change to use iterator
    // let args: Vec<String> = env::args().collect();
    
    // not borrow args, use args() directly
    let config= Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("{} {}",config.query,config.file_path);
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

