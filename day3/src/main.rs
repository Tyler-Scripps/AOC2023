use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    println!("Parsing file: {}", &args[1]);
    let contents = fs::read_to_string(&args[1]).unwrap();

    
}
