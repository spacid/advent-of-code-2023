use aoc::read_lines;

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

fn calculate_result(lines: &str) -> u64 {
    let input: &str = lines.clone();
    let mut sections_split = input.split("\n\n");

    let seeds: Vec<u64> = sections_split
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

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

    let mut destinations: Vec<u64> = vec![];
    for seed in seeds {
        let mut destination = seed;
        for range_section in &range_sections {
            destination = calculate_destination(destination, &range_section);
        }
        destinations.push(destination);
    }
    destinations.iter().min().unwrap().to_owned()
}

fn main() {
    println!(
        "Sum {}",
        calculate_result(&read_lines("inputs/day05.txt").concat())
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fs::read_to_string;
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

        assert_eq!(calculate_result(&lines), 35);
    }

    #[test]
    fn day05a_result() {
        assert_eq!(
            calculate_result(&read_to_string("inputs/day05.txt").unwrap()),
            806029445
        );
    }
}
