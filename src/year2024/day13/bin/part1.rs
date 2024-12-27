use adv_code::year2024::day13::lib::parser::{parse_input, MachineBehavior};
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
        .filter_map(|m| calculate_min_tokens(m))
        .sum()
}

type ButtonACount = isize;

fn calculate_min_tokens(machine: &MachineBehavior) -> Option<isize> {
    let mut min_tokens = None;
    for b_presses in 0isize..100 {
        let Some(a_presses) = calculate_number_of_button_a_presses(machine, b_presses) else {
            continue;
        };
        let tokens = a_presses * 3 + b_presses;
        if min_tokens.is_none() || tokens < min_tokens.unwrap() {
            min_tokens = Some(tokens);
        }
    }
    min_tokens
}

fn calculate_number_of_button_a_presses(
    ((a_x, a_y), (b_x, b_y), (p_x, p_y)): &MachineBehavior,
    number_of_button_b_presses: isize,
) -> Option<ButtonACount> {
    let remaining_x = p_x - b_x * number_of_button_b_presses;
    let remaining_y = p_y - b_y * number_of_button_b_presses;
    if remaining_x % a_x == 0 && remaining_y % a_y == 0 {
        let xa_presses = remaining_x / a_x;
        if xa_presses == remaining_y / a_y {
            return Some(xa_presses.try_into().unwrap());
        }
    }
    None
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
