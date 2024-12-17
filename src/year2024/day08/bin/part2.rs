use adv_code::lib::grid_utils::{Column, GridCoordinates, Row};
use adv_code::year2024::day08::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::{iterate, Itertools};
use std::collections::HashMap;

const INPUT_FILE: &str = "input/2024/08/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 08, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1235);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let grid = parse_input(&file_input_path);

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

    antinodes.count()
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
    use crate::part2;
    use adv_code::year2024::day08::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/08/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 34);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
