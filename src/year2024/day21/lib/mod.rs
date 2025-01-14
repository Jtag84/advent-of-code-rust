use crate::lib::grid_utils::XYCoordinates;
use crate::year2024::day21::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day21::lib::part1::part1;
pub use crate::year2024::day21::lib::part2::part2;
use crate::year2024::day21::lib::KeypadType::{Directional, Numeric};
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use itertools::{repeat_n, Itertools};
use std::collections::HashMap;

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_21_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 21),
    parser: parse_input,
    part1,
    expected_part1: "123096",
    part2,
    expected_part2: "154517692795352",
};

aoc_solver!(YEAR_2024_DAY_21_SOLUTION);

pub fn chains_keypad(code: &str, number_of_robots: usize) -> usize {
    let numeric_keypad = NumericKeypad { arm_position: 'A' };
    let directional_keypad = DirectionalKeyPad { arm_position: 'A' };
    let mut keypads = vec![Numeric(numeric_keypad)];
    repeat_n(Directional(directional_keypad), number_of_robots - 1).for_each(|d| keypads.push(d));
    calculate_shortest_sequence_length(keypads, vec![code.parse().unwrap()], &mut HashMap::new())
}

fn calculate_shortest_sequence_length(
    mut keypads: Vec<KeypadType>,
    directions: Vec<String>,
    cache: &mut HashMap<(Vec<KeypadType>, Vec<String>), usize>,
) -> usize {
    if keypads.is_empty() {
        return directions
            .iter()
            .map(|d| d.len())
            .min()
            .or_else(|| Some(0))
            .unwrap();
    }

    let cached_value = cache.get(&(keypads.clone(), directions.clone()));
    if cached_value.is_some() {
        return cached_value.unwrap().clone();
    }

    let directions_length = directions
        .iter()
        .map(|c| {
            let mut keypad = keypads.get_mut(0).unwrap().clone();
            c.chars()
                .map(|c| keypad.move_arm_to_press_key(c))
                .map(|c| {
                    let next_keypads = keypads.clone().into_iter().skip(1).collect_vec();
                    let result =
                        calculate_shortest_sequence_length(next_keypads.clone(), c.clone(), cache);
                    cache.insert((next_keypads.clone(), c), result.clone());
                    result
                })
                .sum::<usize>()
        })
        .min()
        .unwrap();

    directions_length
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum KeypadType {
    Numeric(NumericKeypad),
    Directional(DirectionalKeyPad),
}

impl Keypad for KeypadType {
    fn move_arm_to_press_key(&mut self, key: char) -> Vec<String> {
        match self {
            Numeric(keypad) => keypad.move_arm_to_press_key(key),
            Directional(keypad) => keypad.move_arm_to_press_key(key),
        }
    }
}

pub trait Keypad: Clone {
    fn move_arm_to_press_key(&mut self, key: char) -> Vec<String>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NumericKeypad {
    arm_position: char,
}

type Move = XYCoordinates;

impl Keypad for NumericKeypad {
    fn move_arm_to_press_key(&mut self, to_key: char) -> Vec<String> {
        let to_key_position = Self::get_position(to_key);
        let from_arm_position = Self::get_position(self.arm_position);
        let move_coordinates = to_key_position - from_arm_position;

        let result = if (self.arm_position == '0' || self.arm_position == 'A')
            && (to_key == '1' || to_key == '4' || to_key == '7')
        {
            vec![move_to_string_vertical_first(move_coordinates)]
        } else if (self.arm_position == '1' || self.arm_position == '4' || self.arm_position == '7')
            && (to_key == '0' || to_key == 'A')
        {
            vec![move_to_string_horizontal_first(move_coordinates)]
        } else {
            vec![
                move_to_string_horizontal_first(move_coordinates),
                move_to_string_vertical_first(move_coordinates),
            ]
            .into_iter()
            .unique()
            .collect_vec()
        };

        self.arm_position = to_key;
        result
    }
}

impl NumericKeypad {
    fn get_position(key: char) -> XYCoordinates {
        match key {
            '7' => XYCoordinates::from((0, 0)),
            '8' => XYCoordinates::from((1, 0)),
            '9' => XYCoordinates::from((2, 0)),
            '4' => XYCoordinates::from((0, 1)),
            '5' => XYCoordinates::from((1, 1)),
            '6' => XYCoordinates::from((2, 1)),
            '1' => XYCoordinates::from((0, 2)),
            '2' => XYCoordinates::from((1, 2)),
            '3' => XYCoordinates::from((2, 2)),
            '0' => XYCoordinates::from((1, 3)),
            'A' => XYCoordinates::from((2, 3)),
            _ => panic!("Unknown key: {}", key),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct DirectionalKeyPad {
    arm_position: char,
}

impl Keypad for DirectionalKeyPad {
    fn move_arm_to_press_key(&mut self, to_key: char) -> Vec<String> {
        let result = match self.arm_position {
            '^' => match to_key {
                'A' => vec![">A"],
                '^' => vec!["A"],
                '<' => vec!["v<A"],
                'v' => vec![">A"],
                '>' => vec!["v>A", ">vA"],
                _ => unreachable!(),
            },
            'A' => match to_key {
                'A' => vec!["A"],
                '^' => vec!["<A"],
                '<' => vec!["v<<A"],
                'v' => vec!["v<A", "<vA"],
                '>' => vec!["vA"],
                _ => unreachable!(),
            },
            '<' => match to_key {
                'A' => vec![">>^A"],
                '^' => vec![">^A"],
                '<' => vec!["A"],
                'v' => vec![">A"],
                '>' => vec![">>A"],
                _ => unreachable!(),
            },
            'v' => match to_key {
                'A' => vec![">^A", "^>A"],
                '^' => vec!["^A"],
                '<' => vec!["<A"],
                'v' => vec!["A"],
                '>' => vec![">A"],
                _ => unreachable!(),
            },
            '>' => match to_key {
                'A' => vec!["^A"],
                '^' => vec!["^<A", "<^A"],
                '<' => vec!["<<A"],
                'v' => vec!["<A"],
                '>' => vec!["A"],
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        self.arm_position = to_key;
        result.iter().map(|s| s.to_string()).collect()
    }
}

pub fn move_to_string_vertical_first(XYCoordinates(horizontal, vertical): Move) -> String {
    let mut result = "".to_string();

    let horizontal_char = if horizontal.is_negative() { '<' } else { '>' };

    let vertical_char = if vertical.is_negative() { '^' } else { 'v' };

    repeat_n(vertical_char, vertical.abs() as usize).for_each(|c| result.push(c));
    repeat_n(horizontal_char, horizontal.abs() as usize).for_each(|c| result.push(c));

    result.push('A');

    result
}

pub fn move_to_string_horizontal_first(XYCoordinates(horizontal, vertical): Move) -> String {
    let mut result = "".to_string();

    let horizontal_char = if horizontal.is_negative() { '<' } else { '>' };

    let vertical_char = if vertical.is_negative() { '^' } else { 'v' };

    repeat_n(horizontal_char, horizontal.abs() as usize).for_each(|c| result.push(c));
    repeat_n(vertical_char, vertical.abs() as usize).for_each(|c| result.push(c));

    result.push('A');

    result
}

#[cfg(test)]
mod test {
    use crate::year2024::day21::lib::KeypadType::{Directional, Numeric};
    use crate::year2024::day21::lib::{
        calculate_shortest_sequence_length, DirectionalKeyPad, Keypad, NumericKeypad,
    };
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn numeric_keypad_press_key() {
        let mut numeric_keypad = NumericKeypad { arm_position: 'A' };
        let directions: String = "029A"
            .chars()
            .map(|c| numeric_keypad.move_arm_to_press_key(c))
            .multi_cartesian_product()
            .map(|c| c.join(""))
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();

        println!("Result = {}", directions);
        assert_eq!(directions, "<A^A>^^AvvvA");
    }

    #[test]
    fn numeric_keypad_press_key_all_alternatives() {
        let mut numeric_keypad = NumericKeypad { arm_position: 'A' };
        let directions_alternatives: Vec<_> = "029A"
            .chars()
            .map(|c| numeric_keypad.move_arm_to_press_key(c))
            .multi_cartesian_product()
            .map(|c| c.join(""))
            .collect();

        println!("Result = {:?}", directions_alternatives);
        assert_eq!(directions_alternatives, ["<A^A>^^AvvvA", "<A^A^^>AvvvA"]);
    }

    #[test]
    fn test_calculate_shortest_sequence_length() {
        let numeric_keypad = NumericKeypad { arm_position: 'A' };
        let directional_keypad = DirectionalKeyPad { arm_position: 'A' };
        let directions_min_length = calculate_shortest_sequence_length(
            vec![Numeric(numeric_keypad), Directional(directional_keypad)],
            vec!["029A".parse().unwrap()],
            &mut HashMap::new(),
        );

        println!("Result = {}", directions_min_length);
        assert_eq!(directions_min_length, 28);
    }
}
