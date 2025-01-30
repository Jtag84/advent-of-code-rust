use crate::lib::grid_utils::XYCoordinates;
use crate::year2016::day13::lib::parser::{parse_input, ParsedInput};
pub use crate::year2016::day13::lib::part1::part1;
pub use crate::year2016::day13::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2016_DAY_13_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2016, 13),
    parser: parse_input,
    part1: |input| part1(input, XYCoordinates(31, 39)),
    expected_part1: "82",
    part2,
    expected_part2: "138",
};

aoc_solver!(YEAR_2016_DAY_13_SOLUTION);
