use crate::lib::grid_utils::{Column, GridCoordinates, Row};
use crate::year2024::day08::lib::parser::ParsedInput;
use itertools::{iterate, Itertools};
use std::collections::HashMap;

pub fn part2(grid: ParsedInput) -> String {
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
            coordinates.iter().combinations(2).flat_map(|antenna_pair| {
                get_antinodes(
                    *antenna_pair[0],
                    *antenna_pair[1],
                    grid.iter_rows().len() as Row,
                    grid.iter_cols().len() as Column,
                )
            })
        })
        .unique()
        .filter(|GridCoordinates(row, column)| grid.get(*row, *column).is_some());

    antinodes.count().to_string()
}

fn get_antinodes(
    antenna_left: GridCoordinates,
    antenna_right: GridCoordinates,
    max_row: Row,
    max_column: Column,
) -> Vec<GridCoordinates> {
    let diff = antenna_left - antenna_right;

    vec![
        iterate(antenna_left, |antenna| *antenna + diff)
            .take_while(|GridCoordinates(row, column)| {
                *row >= 0 && *column >= 0 && *row <= max_row && *column <= max_column
            })
            .collect::<Vec<_>>(),
        iterate(antenna_left, |antenna| *antenna - diff)
            .take_while(|GridCoordinates(row, column)| {
                *row >= 0 && *column >= 0 && *row <= max_row && *column <= max_column
            })
            .collect(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod test {
    use crate::year2024::day08::lib::{part2, YEAR_2024_DAY_08_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_08_SOLUTION.get_parsed_test_inputs(2)),
            "34"
        );
    }
}
