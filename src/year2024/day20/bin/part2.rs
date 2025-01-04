use adv_code::year2024::day20::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use std::iter::zip;

const INPUT_FILE: &str = "input/2024/20/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 20, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1);

    Ok(())
}

fn part2(file_input_path: &str) -> isize {
    let parsed = parse_input(&file_input_path);
    0
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day20::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/20/test_inputs_part2.txt";

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
