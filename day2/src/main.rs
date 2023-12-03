use std::fs;
use std::env;

struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Parsing file: {}", &args[1]);

}
