use adv_code::year2024::day17::lib::parser::parse_input;
use adv_code::year2024::day17::lib::ProgramState;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;

const INPUT_FILE: &str = "input/2024/17/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 17, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, "6,5,7,4,5,7,3,1,0");

    Ok(())
}

fn part1(file_input_path: &str) -> String {
    let (register_a, register_b, register_c, program) = parse_input(&file_input_path);
    let initial_program_state = ProgramState::new(register_a, register_b, register_c, program);
    let final_state = initial_program_state.run_program();
    final_state.output.into_iter().join(",")
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day17::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/17/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
