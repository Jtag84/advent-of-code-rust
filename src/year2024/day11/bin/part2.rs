use adv_code::year2024::day11::lib::blink_n;
use adv_code::year2024::day11::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/11/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 11, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 225404711855335);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let stones = parse_input(&file_input_path);
    blink_n(stones, 75)
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day11::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/11/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 65601038650482);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
