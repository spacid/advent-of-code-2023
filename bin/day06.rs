use std::fs::read_to_string;

fn calculate_result(durations: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut result: u64 = 1;
    for (idx, duration) in durations.iter().enumerate() {
        let mut counter = 0;
        let distance = distances[idx];
        for x in 0..duration.to_owned() + 1 {
            let distance_travelled = x * (duration.to_owned() - x);
            if distance_travelled > distance {
                counter += 1;
            }
        }
        if counter > 0 {
            result *= counter;
        }
    }
    result
}

fn calculate_result_part_1(lines: &str) -> u64 {
    let input: Vec<&str> = lines.lines().collect();
    let durations: Vec<u64> = input[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = input[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    calculate_result(&durations, &distances)
}

fn calculate_result_part_2(lines: &str) -> u64 {
    let input: Vec<&str> = lines.lines().collect();
    let duration: u64 = input[0]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance: u64 = input[1]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    calculate_result(&vec![duration], &vec![distance])
}

fn main() {
    println!(
        "Sum {}",
        calculate_result_part_1(&read_to_string("inputs/day06.txt").unwrap())
    );
    println!(
        "Sum {}",
        calculate_result_part_2(&read_to_string("inputs/day06.txt").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]

    fn day06a_sample() {
        let lines = indoc! {"
    Time:      7  15   30
    Distance:  9  40  200
    "};

        assert_eq!(calculate_result_part_1(&lines), 288);
    }

    #[test]
    fn day06a_result() {
        assert_eq!(
            calculate_result_part_1(&read_to_string("inputs/day06.txt").unwrap()),
            303600
        );
    }

    #[test]
    fn day06b_sample() {
        let lines = indoc! {"
    Time:      7  15   30
    Distance:  9  40  200
    "};

        assert_eq!(calculate_result_part_2(&lines), 71503);
    }

    #[test]
    fn day06b_result() {
        assert_eq!(
            calculate_result_part_2(&read_to_string("inputs/day06.txt").unwrap()),
            303600
        );
    }
}
