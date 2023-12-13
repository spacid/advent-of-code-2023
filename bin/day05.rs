use core::str::Split;
use std::fs::read_to_string;

struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn calculate_destination(seed: u64, ranges: &Vec<Range>) -> u64 {
    for range in ranges {
        if seed >= range.source_range_start && seed < range.source_range_start + range.range_length
        {
            return range.destination_range_start + seed - range.source_range_start;
        }
    }
    // Not found in any range, so return the same value as the seed.
    seed
}
fn calculate_result(seeds: &Vec<u64>, sections_split: Split<'_, &str>) -> u64 {
    let range_sections: Vec<Vec<Range>> = sections_split
        .filter(|x| !x.is_empty())
        .map(|section| {
            let lines = section.lines();
            let ranges: Vec<Range> = lines
                .skip(1)
                .map(|line| {
                    let values: Vec<u64> =
                        line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
                    Range {
                        destination_range_start: values[0],
                        source_range_start: values[1],
                        range_length: values[2],
                    }
                })
                .collect();
            ranges
        })
        .collect();

    seeds
        .iter()
        .map(|seed| {
            let mut lowest_destination: u64 = u64::MAX;
            let mut destination = seed.to_owned();
            for range_section in &range_sections {
                destination = calculate_destination(destination, &range_section);
            }
            if  destination < lowest_destination {
                lowest_destination = destination;
            }
            lowest_destination
        })
        .min()
        .unwrap()
}

fn calculate_result_part_1(lines: &str) -> u64 {
    let input: &str = lines.clone();
    let mut sections_split = input.split("\n\n");

    let seeds: Vec<u64> = sections_split
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    calculate_result(&seeds, sections_split)
}

fn calculate_result_part_2(lines: &str) -> u64 {
    let input: &str = lines.clone();
    let mut sections_split = input.split("\n\n");

    let x: Vec<u64> = sections_split
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pointer = 0;
    let mut lowest_destination = u64::MAX;
    for _ in pointer..x.len() / 2 {
        let partial_seeds: Vec<u64> = (x[pointer]..x[pointer] + x[pointer + 1]).collect();
        let destination = calculate_result(&partial_seeds, sections_split.clone());
        if destination < lowest_destination {
            lowest_destination = destination;
        }
        pointer += 2;
    }

    lowest_destination
}

fn main() {
    println!(
        "Sum {}",
        calculate_result_part_1(&read_to_string("inputs/day05.txt").unwrap())
    );
    println!(
        "Sum {}",
        calculate_result_part_2(&read_to_string("inputs/day05.txt").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn day05a_sample() {
        let lines = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4

    "};

        assert_eq!(calculate_result_part_1(&lines), 35);
    }

    #[test]
    fn day05a_result() {
        assert_eq!(
            calculate_result_part_1(&read_to_string("inputs/day05.txt").unwrap()),
            806029445
        );
    }

    #[test]
    fn day05b_sample() {
        let lines = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4

    "};

        assert_eq!(calculate_result_part_2(&lines), 46);
    }

    #[test]
    fn day05b_result() {
        assert_eq!(
            calculate_result_part_2(&read_to_string("inputs/day05.txt").unwrap()),
            59370572
        );
    }
}
