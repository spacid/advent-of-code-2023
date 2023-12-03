use std::{fs::read_to_string, char};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn calculate_result() -> u32{
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
    let lines  = read_lines("inputs/day01.txt");
    for line in lines {

        let mut sub_line = line;
        for (key, val) in map {
            sub_line = sub_line.replace(key, format!("{key}{val}{key}").as_str());
        }

        let mut char1: char = 'b';
        let mut char2: char ='c';
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
        let digit:u32 = concat_str.parse().unwrap();
        sum += digit;
    }
    sum
}

fn main() {
    println!("Sum: {}", calculate_result())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01b() {
        assert_eq!(calculate_result(), 54845);
    }
}