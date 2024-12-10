use adv_code::year2024::day01::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;

const YEAR: &str = "2024";
const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", YEAR, "/", DAY, "/", "inputs.txt");

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 2 ===");

    let result = time_snippet!(calculate_similarity_score(INPUT_FILE));
    println!("Similarity score = {}", result);

    assert_eq!(result, 22014209);

    Ok(())
}

fn calculate_similarity_score(file_input_path: &str) -> i32 {
    let (left_column, right_column) = parse_input(&file_input_path);

    let right_occurrences = right_column.into_iter()
        .into_group_map_by(|x| *x)
        .into_iter()
        .map(|(key, group)| (key, group.len().try_into().unwrap()))
        .collect::<HashMap<i32, i32>>();

    left_column.into_iter()
        .map(|x| {
            right_occurrences.get(&x)
                .map_or_else(|| 0, 
                             |y| y * x)}
        ).sum()
}

#[cfg(test)]
mod test {
    use crate::{calculate_similarity_score, DAY, YEAR};
    use const_format::concatcp;

    const TEST_INPUT_FILE: &str = concatcp!("input/", YEAR, "/", DAY, "/", "test_inputs_part1.txt");

    #[test]
    fn test_part1_calculate_total_distance() {
        assert_eq!(calculate_similarity_score(TEST_INPUT_FILE), 31);
    }
}
