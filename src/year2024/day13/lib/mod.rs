use crate::year2024::day13::lib::parser::MachineBehavior;
use crate::year2024::day13::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day13::lib::part1::part1;
pub use crate::year2024::day13::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_13_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 13),
    parser: parse_input,
    part1,
    expected_part1: "29877",
    part2,
    expected_part2: "99423413811305",
};

aoc_solver!(YEAR_2024_DAY_13_SOLUTION);

pub type ButtonACount = isize;
pub type ButtonBCount = isize;

pub fn calculate_presses(
    ((a_x, a_y), (b_x, b_y), (p_x, p_y)): &MachineBehavior,
) -> Option<(ButtonACount, ButtonBCount)> {
    // Cramer's rule
    // a * a_x + b * b_x = p_x
    // a * a_y + b * b_y = p_y
    let denominator = a_x * b_y - a_y * b_x;
    if denominator == 0 {
        return None;
    }

    let a_num = p_x * b_y - p_y * b_x;
    let b_num = p_y * a_x - p_x * a_y;
    if a_num % denominator != 0 || b_num % denominator != 0 {
        return None;
    }

    let a_presses = a_num / denominator;
    let b_presses = b_num / denominator;
    if a_presses >= 0 && b_presses >= 0 {
        Some((a_presses, b_presses))
    } else {
        None
    }
}
