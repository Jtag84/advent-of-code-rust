use crate::lib::grid_utils::Direction::East;
use crate::lib::grid_utils::{GridCoordinates, Position};
use crate::year2023::day16::lib::moving_beam;
use crate::year2023::day16::lib::parser::ParsedInput;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(grid: ParsedInput) -> String {
    moving_beam(
        &grid,
        Position(GridCoordinates(0, 0), East),
        &mut HashSet::new(),
    )
    .iter()
    .map(|p| p.coordinates())
    .unique()
    .count()
    .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2023::day16::lib::part1::part1;
    use crate::year2023::day16::lib::YEAR_2023_DAY_16_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2023_DAY_16_SOLUTION.get_parsed_test_inputs(1)),
            "46"
        );
    }
}
