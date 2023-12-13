use aoc::read_lines;

fn calculate_result(lines: Vec<String>) -> u32 {
    let mut scratch_cards: Vec<u32> = vec![1; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let split_line: Vec<&str> = line.split("|").collect();
        let winning_numbers: Vec<u32> = split_line[0]
            .split(":")
            .skip(1) // Skip "Card X:"
            .next() // Get the part with the winning numbers
            .unwrap() // Get the part with the winning numbers
            .split(" ") // Split on space
            .filter(|x| !x.is_empty()) // Basically remove whitespace characters
            .map(|x| x.parse::<u32>().unwrap()) // Parse the number String
            .collect();
        let count: usize = split_line[1]
            .split(" ") // Split on space
            .filter(|x| !x.is_empty()) // Basically remove whitespace characters
            .map(|x| x.parse::<u32>().unwrap()) // Parse the number String
            .filter(|x| winning_numbers.contains(x)) // Check if nummers is in the winning numbers
            .count() // Count the amount of winning numbers
            .try_into() // Try to parse as u32
            .unwrap();

        for relative_idx in (idx + 1)..(idx + count + 1) {
            scratch_cards[relative_idx] += scratch_cards[idx] * 1;
        }
    }
    scratch_cards.iter().sum()
}

fn main() {
    println!("Sum {}", calculate_result(read_lines("inputs/day03.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day04b_sample() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_owned(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_owned(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_owned(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_owned(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_owned(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_owned(),
        ];
        assert_eq!(calculate_result(lines), 30);
    }

    #[test]
    fn day04b_result() {
        assert_eq!(calculate_result(read_lines("inputs/day04.txt")), 5744979);
    }
}
