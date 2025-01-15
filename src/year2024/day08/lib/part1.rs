use crate::lib::grid_utils::GridCoordinates;
use crate::year2024::day08::lib::parser::ParsedInput;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(grid: ParsedInput) -> String {
    let antennas_coordinates: HashMap<&char, Vec<GridCoordinates>> = grid
        .indexed_iter()
        .filter(|(_, &element)| element != '.')
        .map(|(coordinates, element)| (element, GridCoordinates::from(coordinates)))
        .into_group_map()
        .into_iter()
        .collect();

    let antinodes = antennas_coordinates
        .values()
        .flat_map(|coordinates| {
            coordinates
                .iter()
                .combinations(2)
                .flat_map(|antenna_pair| get_antinodes(*antenna_pair[0], *antenna_pair[1]))
        })
        .unique()
        .filter(|GridCoordinates(row, column)| grid.get(*row, *column).is_some());

    antinodes.count().to_string()
}

fn get_antinodes(
    antenna_left: GridCoordinates,
    antenna_right: GridCoordinates,
) -> Vec<GridCoordinates> {
    let diff_right = antenna_left - antenna_right;
    let diff_left = antenna_right - antenna_left;
    vec![antenna_left - diff_left, antenna_right - diff_right]
}

#[cfg(test)]
mod test {
    use crate::year2024::day08::lib::part1::part1;
    use crate::year2024::day08::lib::YEAR_2024_DAY_08_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_08_SOLUTION.get_parsed_test_inputs(1)),
            "14"
        );
    }
}
