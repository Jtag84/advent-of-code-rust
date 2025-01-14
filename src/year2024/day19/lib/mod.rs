use crate::year2024::day19::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day19::lib::part1::part1;
pub use crate::year2024::day19::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_19_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 19),
    parser: parse_input,
    part1,
    expected_part1: "358",
    part2,
    expected_part2: "600639829400603",
};

aoc_solver!(YEAR_2024_DAY_19_SOLUTION);
