use adv_code::year2024::day13::lib::calculate_presses;
use adv_code::year2024::day13::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/13/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 13, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 29877);

    Ok(())
}

fn part1(file_input_path: &str) -> isize {
    let machine_behaviors = parse_input(&file_input_path);
    machine_behaviors
        .iter()
        .filter_map(|m| calculate_presses(m))
        .filter(|(a, b)| *a <= 100 && *b <= 100)
        .map(|(a, b)| 3 * a + b)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day13::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/13/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 480);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
