use aoc::read_lines_as_vec;
struct NumberPos {
    pub start_index: usize,
    pub end_index: usize,
    pub row: usize,
    pub value: u32,
}
struct Boundary {
    pub x_up: usize,
    pub x_down: usize,
    pub y_left: usize,
    pub y_right: usize,
}
struct StarPos {
    pub x: usize,
    pub y: usize,
}

trait CharacterPos {
    fn calculate_boundary(&self, x_boundary: usize, y_boundary: usize) -> Boundary;
}

impl CharacterPos for NumberPos {
    fn calculate_boundary(&self, x_boundary: usize, y_boundary: usize) -> Boundary {
        let x_down = if self.row == 0 { 0 } else { self.row - 1 };
        let x_up = if self.row == x_boundary {
            x_boundary
        } else {
            self.row + 1
        };
        let y_left = if self.start_index == 0 {
            0
        } else {
            self.start_index - 1
        };
        let y_right = if self.end_index == y_boundary {
            y_boundary
        } else {
            self.end_index + 1
        };

        Boundary {
            x_up: x_up,
            x_down: x_down,
            y_left: y_left,
            y_right: y_right,
        }
    }
}

impl NumberPos {
    fn is_star_adjacent(&self, star_pos: &StarPos, x_boundary: usize, y_boundary: usize) -> bool {
        let mut has_adjacent_star: bool = false;
        let boundary = self.calculate_boundary(x_boundary, y_boundary);
        for x_pos in boundary.x_down..boundary.x_up + 1 {
            for y_pos in boundary.y_left..boundary.y_right + 1 {
                if x_pos == star_pos.x && y_pos == star_pos.y {
                    has_adjacent_star = true;
                }
            }
        }
        has_adjacent_star
    }
}

impl CharacterPos for StarPos {
    fn calculate_boundary(&self, x_boundary: usize, y_boundary: usize) -> Boundary {
        let x_down = if self.x == 0 { 0 } else { self.x - 1 };
        let x_up = if self.x == x_boundary {
            x_boundary
        } else {
            self.x + 1
        };
        let y_left = if self.y == 0 { 0 } else { self.y - 1 };
        let y_right = if self.y == y_boundary {
            y_boundary
        } else {
            self.y + 1
        };

        Boundary {
            x_up: x_up,
            x_down: x_down,
            y_left: y_left,
            y_right: y_right,
        }
    }
}

fn calculate_number_positions(line: &Vec<char>, row: usize) -> Vec<NumberPos> {
    let mut line_vec = vec![];
    let mut number_str = String::new();
    let mut start_index = 0;
    for (idx, character) in line.iter().enumerate() {
        if character.is_ascii_digit() {
            if number_str.is_empty() {
                start_index = idx;
            }
            number_str.push(character.to_owned());
        } else {
            if !number_str.is_empty() {
                line_vec.push(NumberPos {
                    start_index: start_index,
                    end_index: idx - 1,
                    value: number_str.parse::<u32>().unwrap(),
                    row: row,
                });
                number_str = String::new();
            }
        }
    }
    // The case when a number ends at the boundary border
    if !number_str.is_empty() {
        line_vec.push(NumberPos {
            start_index: start_index,
            end_index: line.len() - 1,
            value: number_str.parse::<u32>().unwrap(),
            row: row,
        });
    }
    line_vec
}

fn calculate_result_part1(lines: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    // Boundaries of grid, counting from 0
    let x_boundary = lines.len() - 1;
    let y_boundary = lines[0].len() - 1;

    for (line_idx, line) in lines.iter().enumerate() {
        // Get all "number positions" for a given line.
        let numbers_positions = calculate_number_positions(line, line_idx);
        for number_position in numbers_positions {
            let mut part_number = false;
            // Get the grid around a number position
            let boundary = number_position.calculate_boundary(x_boundary, y_boundary);
            // Check if there is a "special" character in the grid around a number position
            for x_pos in boundary.x_down..boundary.x_up + 1 {
                for y_pos in boundary.y_left..boundary.y_right + 1 {
                    let x_y = lines[x_pos][y_pos];
                    if !x_y.is_digit(10) && x_y.to_owned() != '.' {
                        part_number = true;
                    }
                }
            }
            if part_number {
                sum += number_position.value;
            }
        }
    }
    sum
}

fn calculate_result_part2(lines: Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;
    let mut numbers_positions: Vec<Vec<NumberPos>> = vec![];

    // Boundaries of grid, counting from 0
    let x_boundary = lines.len() - 1;
    let y_boundary = lines[0].len() - 1;

    for (line_idx, line) in lines.iter().enumerate() {
        // Get all "number positions" for a given line.
        numbers_positions.push(calculate_number_positions(line, line_idx));
    }

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, char) in line.iter().enumerate() {
            if char == &'*' {
                let star_pos = StarPos {
                    x: line_idx,
                    y: char_idx,
                };
                let mut adjacent_number_positions: Vec<&NumberPos> = vec![];
                let boundary = star_pos.calculate_boundary(x_boundary, y_boundary);
                for x_pos in boundary.x_down..boundary.x_up + 1 {
                    for number_position in &numbers_positions[x_pos] {
                        if number_position.is_star_adjacent(&star_pos, x_boundary, y_boundary) {
                            adjacent_number_positions.push(number_position);
                        }
                    }
                }
                if adjacent_number_positions.len() == 2 {
                    sum += adjacent_number_positions[0].value * adjacent_number_positions[1].value;
                }
            }
        }
    }
    sum
}

fn main() {
    println!(
        "Sum: {}",
        calculate_result_part1(read_lines_as_vec("inputs/day03.txt"))
    );
    println!(
        "Gear: {}",
        calculate_result_part2(read_lines_as_vec("inputs/day03.txt"))
    )
}

#[cfg(test)]
mod tests {
    use super::{calculate_result_part1, calculate_result_part2, read_lines_as_vec};
    #[test]
    fn day03a_sample() {
        let lines = vec![
            "467..114..".chars().collect(),
            "...*......".chars().collect(),
            "..35..633.".chars().collect(),
            "......#...".chars().collect(),
            "617*......".chars().collect(),
            ".....+.58.".chars().collect(),
            "..592.....".chars().collect(),
            "......755.".chars().collect(),
            "...$.*....".chars().collect(),
            ".664.598..".chars().collect(),
        ];
        assert_eq!(calculate_result_part1(lines), 4361);
    }

    #[test]
    fn day03a_result() {
        assert_eq!(
            calculate_result_part1(read_lines_as_vec("inputs/day03.txt")),
            539433
        );
    }

    #[test]
    fn day03b_sample() {
        let lines = vec![
            "467..114..".chars().collect(),
            "...*......".chars().collect(),
            "..35..633.".chars().collect(),
            "......#...".chars().collect(),
            "617*......".chars().collect(),
            ".....+.58.".chars().collect(),
            "..592.....".chars().collect(),
            "......755.".chars().collect(),
            "...$.*....".chars().collect(),
            ".664.598..".chars().collect(),
        ];
        assert_eq!(calculate_result_part2(lines), 467835);
    }

    #[test]
    fn day03b_result() {
        assert_eq!(
            calculate_result_part2(read_lines_as_vec("inputs/day03.txt")),
            75847567
        );
    }
}
