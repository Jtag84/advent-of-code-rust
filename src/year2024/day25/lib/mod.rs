use crate::year2024::day25::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day25::lib::part1::part1;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;

const YEAR_2024_DAY_25_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 25),
    parser: parse_input,
    part1,
    expected_part1: "3671",
    part2: |_| "No part2".to_string(),
    expected_part2: "No part2",
};

aoc_solver!(YEAR_2024_DAY_25_SOLUTION);
