use crate::year2024::day10::lib::find_hiking_trails;
use crate::year2024::day10::lib::parser::ParsedInput;

pub fn part2(grid: ParsedInput) -> String {
    find_hiking_trails(grid)
        .iter()
        .map(|(_, path_count)| path_count)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day10::lib::part2::part2;
    use crate::year2024::day10::lib::YEAR_2024_DAY_10_SOLUTION;

    #[test]
    fn part2_test() {
        let score = part2(YEAR_2024_DAY_10_SOLUTION.get_parsed_test_inputs(2));
        assert_eq!(score, "81");
    }
}
