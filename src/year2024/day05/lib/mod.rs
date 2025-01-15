use crate::year2024::day05::lib::parser::{parse_input, ParsedInput};
use crate::year2024::day05::lib::parser::{Rule, UpdateLine};
pub use crate::year2024::day05::lib::part1::part1;
pub use crate::year2024::day05::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use itertools::Itertools;
use std::collections::HashSet;

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_05_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 05),
    parser: parse_input,
    part1,
    expected_part1: "7307",
    part2,
    expected_part2: "4713",
};

aoc_solver!(YEAR_2024_DAY_05_SOLUTION);

pub fn is_valid(rules: &HashSet<Rule>, update_line: &UpdateLine) -> bool {
    update_line
        .iter()
        .combinations(2)
        .all(|update_combination| {
            !rules.contains(&(*update_combination[1], *update_combination[0]))
        })
}
