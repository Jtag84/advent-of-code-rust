use adv_code::year2024::day03::parser::{part2_parse_input, OperationInstruction};
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use OperationInstruction::{Do, Dont, Mul};

const INPUT_FILE: &str = "input/2024/03/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 03, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 90044227);

    Ok(())
}

fn part2(file_input_path: &str) -> i32 {
    let operation_instructions = part2_parse_input(&file_input_path);

    let mut is_do = true;
    let mut total = 0;
    for operation_instruction in operation_instructions {
        match operation_instruction {
            Mul(l, r) => {
                if is_do {
                    total += l * r
                }
            }
            Do => is_do = true,
            Dont => is_do = false,
        }
    }
    total
}

#[cfg(test)]
mod test {
    use crate::part2;

    const TEST_INPUT_FILE: &str = "input/2024/03/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 48);
    }
}
