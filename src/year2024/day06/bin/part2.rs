use adv_code::year2024::day06::parser::parse_input;
use adv_code::*;
use anyhow::*;
use itertools::Itertools;
use std::iter::zip;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/06/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 06, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1);

    Ok(())
}

fn part2(file_input_path: &str) -> i32 {
    let (parsed) = parse_input(&file_input_path);
    parsed
}

#[cfg(test)]
mod test {
    use crate::parse_input;
    use crate::part2;

    const TEST_INPUT_FILE: &str = "input/2024/06/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 1);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}