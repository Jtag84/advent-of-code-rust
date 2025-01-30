use crate::year2016::day12::lib::parser::{parse_input, ParsedInput};
pub use crate::year2016::day12::lib::part1::part1;
pub use crate::year2016::day12::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2016_DAY_12_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2016, 12),
    parser: parse_input,
    part1,
    expected_part1: "318077",
    part2,
    expected_part2: "9227731",
};

aoc_solver!(YEAR_2016_DAY_12_SOLUTION);
