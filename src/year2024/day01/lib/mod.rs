use crate::year2024::day01::lib::parser::{parse_input, ParsedInput};
use crate::year2024::day01::lib::part1::calculate_total_distance;
use crate::year2024::day01::lib::part2::calculate_similarity_score;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_01_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 01),
    parser: parse_input,
    part1: calculate_total_distance,
    expected_part1: "1938424",
    part2: calculate_similarity_score,
    expected_part2: "22014209",
};

aoc_solver!(YEAR_2024_DAY_01_SOLUTION);
