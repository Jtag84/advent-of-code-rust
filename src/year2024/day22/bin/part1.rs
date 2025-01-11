use adv_code::year2024::day22::lib::parser::parse_input;
use adv_code::year2024::day22::lib::pseudorandom_n;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/22/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 22, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 13004408787);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let secrets = parse_input(&file_input_path);
    secrets
        .iter()
        .map(|secret| pseudorandom_n(*secret, 2000))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{part1, pseudorandom_n};
    use adv_code::year2024::day22::lib::parser::parse_input;
    use adv_code::year2024::day22::lib::pseudorandom;

    const TEST_INPUT_FILE: &str = "input/2024/22/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 37327623);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_pseudorandom() {
        let next_secret = pseudorandom(123);
        assert_eq!(next_secret, 15887950);
    }

    #[test]
    fn test_pseudorandom_n() {
        let next_secret = pseudorandom_n(123, 10);
        assert_eq!(next_secret, 5908254);
    }
}
