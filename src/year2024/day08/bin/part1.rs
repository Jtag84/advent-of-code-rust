use adv_code::lib::grid_utils::GridCoordinates;
use adv_code::year2024::day08::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/2024/08/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 08, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 392);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
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
            coordinates
                .iter()
                .combinations(2)
                .flat_map(|antenna_pair| get_antinodes(*antenna_pair[0], *antenna_pair[1]))
        })
        .unique()
        .filter(|GridCoordinates(row, column)| grid.get(*row, *column).is_some());

    antinodes.count()
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
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day08::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/08/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 14);
    }

    #[test]
    fn test_parser() {
        let grid = parse_input(TEST_INPUT_FILE);
        println!("{}", grid_to_str(&grid));
    }
}
