use adv_code::year2024::day13::lib::calculate_presses;
use adv_code::year2024::day13::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/13/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 13, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 99423413811305);

    Ok(())
}

const ADJUSTMENT: isize = 10_000_000_000_000;

fn part2(file_input_path: &str) -> isize {
    let machine_behaviors = parse_input(&file_input_path);
    machine_behaviors
        .iter()
        .filter_map(|(a, b, (px, py))| {
            calculate_presses(&(*a, *b, (px + ADJUSTMENT, py + ADJUSTMENT)))
        })
        .map(|(a, b)| 3 * a + b)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day13::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/13/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 875318608908);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
