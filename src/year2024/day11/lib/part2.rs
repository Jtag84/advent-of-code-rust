use crate::year2024::day11::lib::blink_n;
use crate::year2024::day11::lib::parser::ParsedInput;

pub fn part2(stones: ParsedInput) -> String {
    blink_n(stones, 75).to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day11::lib::{part2, YEAR_2024_DAY_11_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_11_SOLUTION.get_parsed_test_inputs(2)),
            "65601038650482"
        );
    }
}
