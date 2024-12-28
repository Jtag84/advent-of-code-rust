use crate::year2024::day13::lib::parser::MachineBehavior;

pub mod parser;

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
