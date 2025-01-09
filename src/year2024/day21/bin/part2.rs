use adv_code::year2024::day21::lib::chains_keypad;
use adv_code::year2024::day21::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/21/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 21, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 154517692795352);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let codes = parse_input(&file_input_path);

    let complexity_value: usize = codes
        .iter()
        .map(|code| (code, chains_keypad(code, 26)))
        .map(|(code, directions_length)| code[0..3].parse::<usize>().unwrap() * directions_length)
        .sum();

    println!("result {complexity_value}");
    complexity_value
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day21::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/21/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 154115708116294);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
