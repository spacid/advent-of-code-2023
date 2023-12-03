use aoc::read_lines;
use std::char;

fn calculate_result() -> u32 {
    let mut sum: u32 = 0;

    let lines = read_lines("inputs/day01.txt");
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
    println!("Sum {}", calculate_result());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01a() {
        assert_eq!(calculate_result(), 55090);
    }
}
