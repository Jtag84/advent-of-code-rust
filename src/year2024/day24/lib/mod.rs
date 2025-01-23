use crate::year2024::day24::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day24::lib::part1::part1;
pub use crate::year2024::day24::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_24_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 24),
    parser: parse_input,
    part1,
    expected_part1: "47666458872582",
    part2,
    expected_part2: "dnt,gdf,gwc,jst,mcm,z05,z15,z30",
};

aoc_solver!(YEAR_2024_DAY_24_SOLUTION);
