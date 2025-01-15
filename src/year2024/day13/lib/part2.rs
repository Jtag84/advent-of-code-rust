use crate::year2024::day13::lib::calculate_presses;
use crate::year2024::day13::lib::parser::ParsedInput;

const ADJUSTMENT: isize = 10_000_000_000_000;

pub fn part2(machine_behaviors: ParsedInput) -> String {
    machine_behaviors
        .iter()
        .filter_map(|(a, b, (px, py))| {
            calculate_presses(&(*a, *b, (px + ADJUSTMENT, py + ADJUSTMENT)))
        })
        .map(|(a, b)| 3 * a + b)
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day13::lib::part2::part2;
    use crate::year2024::day13::lib::YEAR_2024_DAY_13_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_13_SOLUTION.get_parsed_test_inputs(2)),
            "875318608908"
        );
    }
}
