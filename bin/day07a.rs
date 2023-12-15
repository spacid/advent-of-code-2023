use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read_to_string;

struct Hand {
    pub hand: String,
}

impl Hand {
    fn calculate_type(&self) -> u8 {
        let mut character_count: HashMap<char, u8> = HashMap::new();
        for character in self.hand.chars() {
            character_count
                .entry(character)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut counted_chars: HashMap<u8, Vec<char>> = HashMap::new();
        for (char, count) in character_count {
            counted_chars
                .entry(count)
                .and_modify(|chars| chars.push(char))
                .or_insert(vec![char]);
        }
        // Five of a kind
        if counted_chars.contains_key(&5) {
            return 1;
        }
        // Four of a kind
        else if counted_chars.contains_key(&4) {
            return 2;
        } else if counted_chars.contains_key(&3) {
            // Full house
            if counted_chars.contains_key(&2) {
                return 3;
            }
            // Three of kind
            else {
                return 4;
            }
        } else if counted_chars.contains_key(&2) {
            // Two pair
            if counted_chars.get(&2).unwrap().len() > 1 {
                return 5;
            }
            // Pair
            return 6;
        } else {
            // High card
            return 7;
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let char_map: HashMap<char, i32> = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]);
        let hand_type = self.calculate_type();
        let other_hand_type = other.calculate_type();

        if hand_type < other_hand_type {
            return Ordering::Greater;
        } else if hand_type > other_hand_type {
            return Ordering::Less;
        } else {
            let other_chars: Vec<char> = other.hand.chars().collect();
            for (idx, char) in self.hand.chars().enumerate() {
                let char_value = char_map.get(&char).unwrap();
                let other_char_value = char_map.get(&other_chars[idx]).unwrap();

                if char_value > other_char_value {
                    return Ordering::Greater;
                } else if char_value < other_char_value {
                    return Ordering::Less;
                } else {
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let hand_type = self.calculate_type();
        let other_hand_type = other.calculate_type();
        hand_type == other_hand_type
    }
}

impl Eq for Hand {}

fn calculate_result(lines: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut character_count: BTreeMap<Hand, u32> = BTreeMap::new();
    for line in lines.lines() {
        let mut line_split = line.split_whitespace();
        character_count.insert(
            Hand {
                hand: line_split.next().unwrap().to_owned(),
            },
            line_split.next().unwrap().parse::<u32>().unwrap(),
        );
    }
    let amount_of_hands: u32 = character_count.len().try_into().unwrap();

    for (idx, (_, number)) in character_count.iter().rev().enumerate() {
        sum += (amount_of_hands - (idx as u32)) * number;
    }
    sum
}

fn main() {
    println!(
        "Sum {}",
        calculate_result(&read_to_string("inputs/day07.txt").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn hand_type() {
        // Five of a kind
        assert_eq!(
            Hand {
                hand: "KKKKK".to_string()
            }
            .calculate_type(),
            1
        );
        // Four of a kind
        assert_eq!(
            Hand {
                hand: "KKKK2".to_string()
            }
            .calculate_type(),
            2
        );
        // Full house
        assert_eq!(
            Hand {
                hand: "KKK11".to_string()
            }
            .calculate_type(),
            3
        );
        // Three of a kind
        assert_eq!(
            Hand {
                hand: "KKK87".to_string()
            }
            .calculate_type(),
            4
        );
        // Two pair
        assert_eq!(
            Hand {
                hand: "KKJJ9".to_string()
            }
            .calculate_type(),
            5
        );
        // Pair
        assert_eq!(
            Hand {
                hand: "KK123".to_string()
            }
            .calculate_type(),
            6
        );
        // High card
        assert_eq!(
            Hand {
                hand: "12345".to_string()
            }
            .calculate_type(),
            7
        );
    }

    #[test]
    fn hand_comparison() {
        assert!(
            Hand {
                hand: "KKKKK".to_string()
            } > Hand {
                hand: "KKKKJ".to_string()
            }
        );

        assert!(
            Hand {
                hand: "3QQQQ".to_string()
            } > Hand {
                hand: "2KKKK".to_string()
            }
        );

        assert!(
            Hand {
                hand: "KKKKK".to_string()
            } == Hand {
                hand: "KKKKK".to_string()
            }
        );
    }

    #[test]
    fn day07a_sample() {
        let lines = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};
        assert_eq!(calculate_result(lines), 6440);
    }

    #[test]
    fn day07a_result() {
        assert_eq!(
            calculate_result(&read_to_string("inputs/day07.txt").unwrap()),
            253603890
        );
    }
}
