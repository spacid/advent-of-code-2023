// 12 red cubes, 13 green cubes, and 14 blue cubes
use aoc::read_lines;

fn calculate_result(lines: Vec<String>) -> u32 {
    /* Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green*/

    let mut power: u32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let bag_picks = parts[1].split(";");

        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;

        for bag_pick in bag_picks {
            for b in bag_pick.split(",") {
                let number_color: Vec<&str> = b.split(" ").collect();
                let number = number_color[1].parse::<u32>().unwrap();
                match number_color[2] {
                    "green" => {
                        if number > green {
                            green = number
                        }
                    }
                    "blue" => {
                        if number > blue {
                            blue = number
                        }
                    }
                    "red" => {
                        if number > red {
                            red = number
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        power += green * blue * red;
    }
    power
}

fn main() {
    println!(
        "power: {}",
        calculate_result(read_lines("inputs/day02.txt"))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01b() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_owned(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_owned(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_owned(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_owned(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_owned(),
        ];
        assert_eq!(calculate_result(lines), 2286);
    }
}
