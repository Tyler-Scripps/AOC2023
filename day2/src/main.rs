use std::fs;
use std::env;

struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Parsing file: {}", &args[1]);
    let contents = read_lines(&args[1]);

    let mut total = 0;
    let mut total_power = 0;

    // let line1 = &contents[0];
    // println!("{}", line1);

    for line in contents {
        let game_id_temp = line.split(' ').collect::<Vec<&str>>()[1];
        let mut game_id = game_id_temp.to_string();
        game_id.pop();
        println!("{}", line);
        // println!("{}", game_id);
        
        let hands_str = line.split(':').collect::<Vec<&str>>()[1];
        let mut good_game = true;
        let mut hands: Vec<Hand> = vec![];
        for hand in hands_str.split(";") {
            let mut temp_hand = Hand {
                red: 0,
                green: 0,
                blue: 0,
            };
            for color_num in hand.split(',') {
                let num: i32 = color_num.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap();
                let color = color_num.split(' ').collect::<Vec<&str>>()[2];
                match color {
                    "red" => temp_hand.red = num,
                    "green" => temp_hand.green = num,
                    "blue" => temp_hand.blue = num,
                    _ => println!("bad color name: {}", color),
                }
            }
            if temp_hand.red > 12 || temp_hand.green > 13 || temp_hand.blue > 14 {
                good_game = false;
            }
            hands.push(temp_hand);
        }
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for hand in hands {
            if hand.red > min_red {
                min_red = hand.red;
            }
            if hand.green > min_green {
                min_green = hand.green;
            }
            if hand.blue > min_blue {
                min_blue = hand.blue;
            }
        }
        total_power += min_red * min_green * min_blue;
        if good_game {
            total += game_id.parse::<i32>().unwrap();
            println!("adding to total, new total: {}", total);
        }
    }
    println!("Total: {}", total);
    println!("Total power: {}", total_power);
}
