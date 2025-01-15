use crate::year2024::day10::lib::find_hiking_trails;
use crate::year2024::day10::lib::parser::ParsedInput;

pub fn part1(grid: ParsedInput) -> String {
    find_hiking_trails(grid)
        .iter()
        .map(|(nine_count, _)| nine_count)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day10::lib::part1::part1;
    use crate::year2024::day10::lib::YEAR_2024_DAY_10_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_10_SOLUTION.get_parsed_test_inputs(1)),
            "36"
        );
    }
}
