// 12 red cubes, 13 green cubes, and 14 blue cubes
use aoc::read_lines;
use std::collections::HashMap;

fn calculate_result(lines: Vec<String>) -> u32 {
    /* Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green*/
    let map = HashMap::from([("red", "12"), ("green", "13"), ("blue", "14")]);

    let mut sum: u32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let x: Vec<&str> = parts[0].split(" ").collect();
        let game_id = x[1];
        let bag_picks = parts[1].split(";");
        let mut possible = true;
        for bag_pick in bag_picks {
            for b in bag_pick.split(",") {
                let number_color: Vec<&str> = b.split(" ").collect();

                match map.get(number_color[2]) {
                    Some(&amount) => {
                        if number_color[1].parse::<u32>().unwrap() > amount.parse::<u32>().unwrap()
                        {
                            possible = false;
                            break;
                        }
                    }
                    _ => println!("Don't have Daniel's number."),
                }
            }
            if possible == false {
                break;
            }
        }
        if possible == true {
            sum += game_id.parse::<u32>().unwrap();
        }
    }
    sum
}

fn main() {
    println!("Sum: {}", calculate_result(read_lines("inputs/day02.txt")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day02a_sample() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_owned(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_owned(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_owned(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_owned(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_owned(),
        ];
        assert_eq!(calculate_result(lines), 8);
    }

    #[test]
    fn day02a_result() {
        assert_eq!(calculate_result(read_lines("inputs/day02.txt")), 2105);
    }
}
