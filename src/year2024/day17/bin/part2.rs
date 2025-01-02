use adv_code::year2024::day17::lib::parser::{parse_input, Program, RegisterA};
use adv_code::year2024::day17::lib::ProgramState;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/17/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 17, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 105875099912602);

    Ok(())
}

fn part2(file_input_path: &str) -> RegisterA {
    let (_, _, _, program) = parse_input(&file_input_path);
    let index = program.len() - 1;
    find_a(&program, index, 0).unwrap()
}

fn find_a(program: &Program, index: usize, current_a: RegisterA) -> Option<RegisterA> {
    let shifted_current_a = current_a << 3;
    for a in 0..8isize {
        let new_a = shifted_current_a | a;
        let initial_program_state = ProgramState::new(new_a, 0, 0, program.clone());
        let Some(first_output) = initial_program_state.run_until_first_output() else {
            panic!("error");
        };

        if first_output != program[index] {
            continue;
        }

        if index == 0 {
            return Some(new_a);
        } else if let Some(new_a) = find_a(program, index - 1, new_a) {
            return Some(new_a);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use adv_code::year2024::day17::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/17/test_inputs_part2.txt";

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
