use std::{fs::read_to_string, char};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn calculate_result() -> u32 {
    let mut sum: u32 = 0;

    let lines  = read_lines("inputs/day01.txt");
    for line in lines {
        let mut char1: char = 'b';
        let mut char2: char ='c';
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
        let digit:u32 = concat_str.parse().unwrap();
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