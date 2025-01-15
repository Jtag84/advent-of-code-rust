use crate::year2024::day07::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day07::lib::part1::part1;
pub use crate::year2024::day07::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_07_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 07),
    parser: parse_input,
    part1,
    expected_part1: "21572148763543",
    part2,
    expected_part2: "581941094529163",
};

aoc_solver!(YEAR_2024_DAY_07_SOLUTION);
