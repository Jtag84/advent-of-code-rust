use crate::year2016::day10::lib::parser::{parse_input, ParsedInput};
pub use crate::year2016::day10::lib::part1::part1;
pub use crate::year2016::day10::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2016_DAY_10_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2016, 10),
    parser: parse_input,
    part1: |input| part1(input, vec![61, 17]),
    expected_part1: "73",
    part2,
    expected_part2: "3965",
};

aoc_solver!(YEAR_2016_DAY_10_SOLUTION);
