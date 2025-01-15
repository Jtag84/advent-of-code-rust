use crate::year2024::day06::lib::get_path;
use crate::year2024::day06::lib::parser::ParsedInput;
use std::collections::HashSet;

pub fn part1((guard_position, grid): ParsedInput) -> String {
    let (path, _) = get_path(guard_position, &grid);
    path.iter()
        .map(|(guard_coordinates, _)| guard_coordinates)
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day06::lib::{part1, YEAR_2024_DAY_06_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_06_SOLUTION.get_parsed_test_inputs(1)),
            "41"
        );
    }
}
