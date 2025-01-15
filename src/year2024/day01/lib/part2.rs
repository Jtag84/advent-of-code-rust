use crate::year2024::day01::lib::parser::ParsedInput;
use itertools::Itertools;

pub fn calculate_similarity_score((left_column, right_column): ParsedInput) -> String {
    let right_occurrences = right_column.into_iter().counts();

    left_column
        .into_iter()
        .map(|x| {
            right_occurrences
                .get(&x)
                .map_or_else(|| 0, |y| (*y as i32) * x)
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day01::lib::part2::calculate_similarity_score;
    use crate::year2024::day01::lib::YEAR_2024_DAY_01_SOLUTION;

    #[test]
    fn test_part1_calculate_total_distance() {
        assert_eq!(
            calculate_similarity_score(YEAR_2024_DAY_01_SOLUTION.get_parsed_test_inputs(2)),
            "31"
        );
    }
}
