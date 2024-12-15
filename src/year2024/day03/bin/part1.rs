use adv_code::year2024::day03::parser::part1_parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/03/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 03, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 184511516);

    Ok(())
}

fn part1(file_input_path: &str) -> i32 {
    let muls = part1_parse_input(&file_input_path);
    muls.iter().map(|(left, right)| left * right).sum()
}

#[cfg(test)]
mod test {
    use crate::part1;

    const TEST_INPUT_FILE: &str = "input/2024/03/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 161);
    }
}