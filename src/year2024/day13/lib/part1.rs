use crate::year2024::day13::lib::calculate_presses;
use crate::year2024::day13::lib::parser::ParsedInput;

pub fn part1(machine_behaviors: ParsedInput) -> String {
    machine_behaviors
        .iter()
        .filter_map(|m| calculate_presses(m))
        .filter(|(a, b)| *a <= 100 && *b <= 100)
        .map(|(a, b)| 3 * a + b)
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day13::lib::{part1, YEAR_2024_DAY_13_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_13_SOLUTION.get_parsed_test_inputs(1)),
            "480"
        );
    }
}
