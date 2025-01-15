use crate::year2024::day15::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day15::lib::part1::part1;
pub use crate::year2024::day15::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_15_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 15),
    parser: parse_input,
    part1,
    expected_part1: "1563092",
    part2,
    expected_part2: "1582688",
};

aoc_solver!(YEAR_2024_DAY_15_SOLUTION);
