use crate::lib::grid_utils::{GridCoordinates, Position};
use crate::year2024::day16::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day16::lib::part1::part1;
pub use crate::year2024::day16::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use grid::Grid;

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_16_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 16),
    parser: parse_input,
    part1,
    expected_part1: "143580",
    part2,
    expected_part2: "645",
};

aoc_solver!(YEAR_2024_DAY_16_SOLUTION);

pub type Cost = usize;

pub fn successors(
    grid: &Grid<char>,
    position: &Position<GridCoordinates>,
) -> Vec<(Position<GridCoordinates>, Cost)> {
    vec![
        position
            .move1()
            .filter(|p| {
                grid.get(p.coordinates().row(), p.coordinates().column())
                    .filter(|v| **v == '.')
                    .is_some()
            })
            .map(|p| (p, 1)),
        Some((position.rotate_clockwise_90(), 1000)),
        Some((position.rotate_counter_clockwise_90(), 1000)),
    ]
    .iter()
    .filter_map(|p| *p)
    .collect()
}
