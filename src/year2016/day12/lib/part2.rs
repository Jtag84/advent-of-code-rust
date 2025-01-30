use crate::year2016::day12::lib::parser::ParsedInput;
use crate::year2016::day12::lib::parser::Register::{A, C};
use crate::year2016::day12::lib::part1::State;

pub fn part2(instructions: ParsedInput) -> String {
    let mut state = State::new(instructions);
    state.registers[C] = 1;
    state.run();

    state.registers[A].to_string()
}

#[cfg(test)]
mod test {
    use crate::year2016::day12::lib::part2::part2;
    use crate::year2016::day12::lib::YEAR_2016_DAY_12_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2016_DAY_12_SOLUTION.get_parsed_test_inputs(2)),
            "42"
        );
    }
}
