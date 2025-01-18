use crate::lib::grid_utils::Direction::{East, North, South, West};
use crate::lib::grid_utils::{GridCoordinates, Position};
use crate::year2023::day16::lib::parser::{parse_input, ParsedInput};
pub use crate::year2023::day16::lib::part1::part1;
pub use crate::year2023::day16::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use itertools::Itertools;
use std::collections::HashSet;

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2023_DAY_16_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2023, 16),
    parser: parse_input,
    part1,
    expected_part1: "6883",
    part2,
    expected_part2: "7228",
};

aoc_solver!(YEAR_2023_DAY_16_SOLUTION);

pub fn moving_beam(
    grid: &ParsedInput,
    position @ Position(coordinates, direction): Position<GridCoordinates>,
    visited_positions: &mut HashSet<Position<GridCoordinates>>,
) -> Vec<Position<GridCoordinates>> {
    if let Some(current_grid_element) = coordinates.get_grid_element(grid) {
        if visited_positions.contains(&position) {
            return vec![];
        }

        visited_positions.insert(position);
        let next_positions = match current_grid_element {
            '.' => move_position_or_empty(position),
            '\\' => match direction {
                East => move_position_or_empty(Position(coordinates, South)),
                North => move_position_or_empty(Position(coordinates, West)),
                West => move_position_or_empty(Position(coordinates, North)),
                South => move_position_or_empty(Position(coordinates, East)),
                _ => panic!("Unexpected direction"),
            },
            '/' => match direction {
                East => move_position_or_empty(Position(coordinates, North)),
                North => move_position_or_empty(Position(coordinates, East)),
                West => move_position_or_empty(Position(coordinates, South)),
                South => move_position_or_empty(Position(coordinates, West)),
                _ => panic!("Unexpected direction"),
            },
            '|' => match direction {
                East | West => vec![
                    move_position_or_empty(Position(coordinates, North)),
                    move_position_or_empty(Position(coordinates, South)),
                ]
                .into_iter()
                .flatten()
                .collect(),
                North | South => move_position_or_empty(position),
                _ => panic!("Unexpected direction"),
            },
            '-' => match direction {
                West | East => move_position_or_empty(position),
                North | South => vec![
                    move_position_or_empty(Position(coordinates, East)),
                    move_position_or_empty(Position(coordinates, West)),
                ]
                .into_iter()
                .flatten()
                .collect(),
                _ => panic!("Unexpected direction"),
            },
            _ => panic!("Unexpected element {:?}", current_grid_element),
        };
        vec![
            vec![position],
            next_positions
                .iter()
                .flat_map(|position| moving_beam(grid, position.clone(), visited_positions))
                .collect_vec(),
        ]
        .concat()
    } else {
        vec![]
    }
}

fn move_position_or_empty(position: Position<GridCoordinates>) -> Vec<Position<GridCoordinates>> {
    position.move1().map(|p| vec![p]).unwrap_or(vec![])
}
