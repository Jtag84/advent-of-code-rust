use crate::lib::grid_utils::Coordinates;
use crate::year2024::day06::lib::parser::GuardPosition;
use crate::year2024::day06::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day06::lib::part1::part1;
pub use crate::year2024::day06::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use grid::Grid;
use std::collections::{HashSet, VecDeque};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_06_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 06),
    parser: parse_input,
    part1,
    expected_part1: "5461",
    part2,
    expected_part2: "1836",
};

aoc_solver!(YEAR_2024_DAY_06_SOLUTION);

#[derive(Debug, PartialEq, Eq)]
pub enum Termination {
    ExitedMap,
    Loop,
}

pub fn get_path(
    (start_guard_coordinates, start_direction): GuardPosition,
    grid: &Grid<char>,
) -> (VecDeque<GuardPosition>, Termination) {
    let mut path = VecDeque::new();
    let mut visited = HashSet::new();

    let mut guard_coordinates = start_guard_coordinates;
    let mut direction = start_direction;
    loop {
        let guard_position = (guard_coordinates, direction);
        if !visited.insert((guard_position, direction)) {
            return (path, Termination::Loop);
        }
        path.push_back(guard_position.clone());

        let new_coordinates_option = guard_coordinates.move_to(direction);
        let new_coordinates = match new_coordinates_option {
            Some(coordinates) => coordinates,
            None => {
                return (path, Termination::ExitedMap);
            }
        };

        let map_element = grid.get(new_coordinates.row(), new_coordinates.column());
        match map_element {
            Some('.') => {
                guard_coordinates = new_coordinates;
            }
            Some('#') => {
                direction = direction.rotate_clockwise_90();
            }
            Some(_) => panic!("Unexpected element in map"),
            None => {
                return (path, Termination::ExitedMap);
            }
        }
    }
}
