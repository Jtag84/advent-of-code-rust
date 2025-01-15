use crate::lib::grid_utils::XYCoordinates;
use crate::year2024::day14::lib::parser::PositionAndSpeed;
use crate::year2024::day14::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day14::lib::part1::part1;
pub use crate::year2024::day14::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_14_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 14),
    parser: parse_input,
    part1: |parsed_input| part1(parsed_input, 101, 103),
    expected_part1: "218619324",
    part2: |parsed_input| part2(parsed_input, 101, 103),
    expected_part2: "6446",
};

aoc_solver!(YEAR_2024_DAY_14_SOLUTION);

pub fn calculate_robots_at_n_seconds(
    map_width: isize,
    map_height: isize,
    robots: &Vec<PositionAndSpeed>,
    n: isize,
) -> Vec<XYCoordinates> {
    robots
        .iter()
        .map(|robot| calculate_position_at_n_seconds(robot, map_width, map_height, n))
        .collect::<Vec<XYCoordinates>>()
}

pub fn calculate_position_at_n_seconds(
    (XYCoordinates(x, y), XYCoordinates(vx, vy)): &PositionAndSpeed,
    map_width: isize,
    map_height: isize,
    n: isize,
) -> XYCoordinates {
    XYCoordinates(
        (x + vx * n).rem_euclid(map_width),
        (y + vy * n).rem_euclid(map_height),
    )
}
