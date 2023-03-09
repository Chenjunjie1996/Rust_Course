use std::env;
use std::fs;
use std::process;
use rust3::Config;

fn main() {
    // 3.1
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    // 3.2
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = rust3::run(config) {
        // --snip--
        println!("Application error: {e}");
        process::exit(1);
    }
}
