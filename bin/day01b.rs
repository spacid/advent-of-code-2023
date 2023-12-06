use aoc::read_lines;
use std::char;

fn calculate_result(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    let map = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    for line in lines {
        let mut sub_line = line;
        for (key, val) in map {
            sub_line = sub_line.replace(key, format!("{key}{val}{key}").as_str());
        }

        let mut char1: char = 'b';
        let mut char2: char = 'c';
        for c in sub_line.chars() {
            if c.is_digit(10) {
                char1 = c;
                break;
            }
        }
        for c in sub_line.chars().rev() {
            if c.is_digit(10) {
                char2 = c;
                break;
            }
        }
        let mut concat_str = String::new();
        concat_str.push(char1);
        concat_str.push(char2);
        let digit: u32 = concat_str.parse().unwrap();
        sum += digit;
    }
    sum
}

fn main() {
    println!("Sum: {}", calculate_result(read_lines("inputs/day01.txt")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01b_sample() {
        let lines = vec![
            "two1nine".to_owned(),
            "eightwothree".to_owned(),
            "abcone2threexyz".to_owned(),
            "xtwone3four".to_owned(),
            "4nineeightseven2".to_owned(),
            "zoneight234".to_owned(),
            "7pqrstsixteen".to_owned(),
        ];
        assert_eq!(calculate_result(lines), 281);
    }

    #[test]
    fn day01b_result() {
        assert_eq!(calculate_result(read_lines("inputs/day01.txt")), 54845);
    }
}
