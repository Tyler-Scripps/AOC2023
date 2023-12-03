use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let number_strings = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string()];

    println!("Parsing file: {}", &args[1]);
    let mut contents = fs::read_to_string(&args[1]).unwrap();
    // println!("{}", contents);

    contents = contents.replace("one", "o1ne");
    contents = contents.replace("two", "t2wo");
    contents = contents.replace("three", "t3hree");
    contents = contents.replace("four", "f4our");
    contents = contents.replace("five", "f5ive");
    contents = contents.replace("six", "s6ix");
    contents = contents.replace("seven", "s7even");
    contents = contents.replace("eight", "e8ight");
    contents = contents.replace("nine", "n9ine");

    // println!("{}", contents);

    let mut total:i32 = 0;

    for line in contents.lines() {
        let mut first_num:char = 'a';
        // let mut first_index = 0;
        let mut last_num:char = 'a';
        // let mut last_index = 0;

        for char in line.chars() {
            if char.is_numeric() {
                if first_num == 'a' {
                    first_num = char;
                }
                last_num = char;
            }
        }
        let mut num_str = String::from(first_num);
        num_str.push(last_num);
        // println!("{}", num_str);

        total += num_str.parse::<i32>().unwrap()
    }

    println!("Total: {}", total);
}
