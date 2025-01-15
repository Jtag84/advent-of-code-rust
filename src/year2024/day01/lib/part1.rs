use crate::year2024::day01::lib::parser::ParsedInput;
use itertools::Itertools;
use std::iter::zip;

pub fn calculate_total_distance((left_column, right_column): ParsedInput) -> String {
    zip(left_column.iter().sorted(), right_column.iter().sorted())
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day01::lib::part1::calculate_total_distance;
    use crate::year2024::day01::lib::YEAR_2024_DAY_01_SOLUTION;

    #[test]
    fn test_part1_calculate_total_distance() {
        assert_eq!(
            calculate_total_distance(YEAR_2024_DAY_01_SOLUTION.get_parsed_test_inputs(1)),
            "11"
        );
    }
}
