use aoc::read_lines;
use std::char;

fn calculate_result(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let mut char1: char = 'b';
        let mut char2: char = 'c';
        for c in line.chars() {
            if c.is_digit(10) {
                char1 = c;
                break;
            }
        }
        for c in line.chars().rev() {
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
    println!("Sum {}", calculate_result(read_lines("inputs/day01.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01a_sample() {
        let lines = vec![
            "1abc2".to_owned(),
            "pqr3stu8vwx".to_owned(),
            "a1b2c3d4e5f".to_owned(),
            "treb7uchet".to_owned(),
        ];
        assert_eq!(calculate_result(lines), 142);
    }

    #[test]
    fn day01a_result() {
        assert_eq!(calculate_result(read_lines("inputs/day01.txt")), 55090);
    }
}
