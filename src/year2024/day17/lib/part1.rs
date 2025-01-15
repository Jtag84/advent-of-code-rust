use crate::year2024::day17::lib::parser::ParsedInput;
use crate::year2024::day17::lib::ProgramState;
use itertools::Itertools;

pub fn part1((register_a, register_b, register_c, program): ParsedInput) -> String {
    let initial_program_state = ProgramState::new(register_a, register_b, register_c, program);
    let final_state = initial_program_state.run_program();
    final_state.output.into_iter().join(",")
}

#[cfg(test)]
mod test {
    use crate::year2024::day17::lib::part1::part1;
    use crate::year2024::day17::lib::YEAR_2024_DAY_17_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_17_SOLUTION.get_parsed_test_inputs(1)),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
