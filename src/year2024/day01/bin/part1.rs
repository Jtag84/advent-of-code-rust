use adv_code::year2024::day01::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use std::iter::zip;

const YEAR: &str = "2024";
const DAY: &str = "01";
const INPUT_FILE: &str = "input/2024/01/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 1, 1);

    let result = time_snippet!(calculate_total_distance(INPUT_FILE));
    println!("Total distance = {}", result);

    assert_eq!(result, 1938424);

    Ok(())
}

fn calculate_total_distance(file_input_path: &str) -> i32 {
    let (left_column, right_column) = parse_input(&file_input_path);

    zip(left_column.iter().sorted(), right_column.iter().sorted())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::calculate_total_distance;

    const TEST_INPUT_FILE: &str = "input/2024/01/test_inputs_part1.txt";

    #[test]
    fn test_part1_calculate_total_distance() {
        assert_eq!(calculate_total_distance(TEST_INPUT_FILE), 11);
    }
}
