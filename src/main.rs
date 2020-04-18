
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query,filename)=parse_config(&args);

    println!("in file {}", filename);

    let contents= fs::read_to_string(filename)
        .expect("something went wrong the file can't be read!");
    
    println!("with text: \n{}",contents);
    
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
