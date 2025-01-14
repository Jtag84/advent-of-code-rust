use crate::year2024::day20::lib::parser::{parse_input, ParsedInput};
use crate::year2024::day20::lib::part1::part1;
use crate::year2024::day20::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_20_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 20),
    parser: parse_input,
    part1: |input| part1(input, 100),
    expected_part1: "1343",
    part2: |input| part2(input, 100),
    expected_part2: "982891",
};

aoc_solver!(YEAR_2024_DAY_20_SOLUTION);
