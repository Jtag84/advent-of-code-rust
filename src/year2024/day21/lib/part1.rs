use crate::year2024::day21::lib::chains_keypad;
use crate::year2024::day21::lib::parser::ParsedInput;

pub fn part1(codes: ParsedInput) -> String {
    let complexity_value: usize = codes
        .iter()
        .map(|code| (code, chains_keypad(code, 3)))
        .map(|(code, directions_length)| code[0..3].parse::<usize>().unwrap() * directions_length)
        .sum();

    complexity_value.to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day21::lib::part1;
    use crate::year2024::day21::lib::YEAR_2024_DAY_21_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_21_SOLUTION.get_parsed_test_inputs(1)),
            "126384"
        );
    }
}
