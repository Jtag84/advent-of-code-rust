use crate::year2016::day11::lib::parser::{parse_input, ParsedInput};
pub use crate::year2016::day11::lib::part1::part1;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;

const YEAR_2016_DAY_11_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2016, 11),
    parser: parse_input,
    part1: |_| {
        let floors = [(3, 5), (2, 0), (0, 0), (0, 0)];
        part1(floors)
    },
    expected_part1: "47",
    part2: |_| {
        let floors = [(5, 7), (2, 0), (0, 0), (0, 0)];
        part1(floors)
    },
    expected_part2: "71",
};

aoc_solver!(YEAR_2016_DAY_11_SOLUTION);
