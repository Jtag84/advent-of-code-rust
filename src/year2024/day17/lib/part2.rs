use crate::year2024::day17::lib::parser::{ParsedInput, Program, RegisterA};
use crate::year2024::day17::lib::ProgramState;

pub fn part2((_, _, _, program): ParsedInput) -> String {
    let index = program.len() - 1;
    find_a(&program, index, 0).unwrap().to_string()
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
