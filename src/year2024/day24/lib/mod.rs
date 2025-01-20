use crate::year2024::day24::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day24::lib::part1::part1;
pub use crate::year2024::day24::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use itertools::{iterate, Itertools};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_24_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 24),
    parser: parse_input,
    part1,
    expected_part1: "47666458872582",
    part2: |input| {
        part2(
            input,
            |a, b| a + b,
            iterate((0usize, 1usize), |(x, y)| (*x, y << 1))
                .take(45)
                .flat_map(|(x, y)| [(x, y), (y, x)])
                .collect_vec(),
        )
    },
    expected_part2: "Don't know yet",
};

aoc_solver!(YEAR_2024_DAY_24_SOLUTION);
