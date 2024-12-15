use adv_code::year2024::day05::lib::is_valid;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use adv_code::year2024::day05::lib::parser::parse_input;

const INPUT_FILE: &str = "input/2024/05/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 05, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 7307);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let (rules, updates) = parse_input(&file_input_path);

    updates.iter()
        .filter(|update_line| is_valid(&rules, update_line))
        .map(|update_line| update_line[update_line.len()/2])
        .sum()
}

#[cfg(test)]
mod test {
    use crate::parse_input;
    use crate::part1;

    const TEST_INPUT_FILE: &str = "input/2024/05/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        let result = part1(TEST_INPUT_FILE);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}