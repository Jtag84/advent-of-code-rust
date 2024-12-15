use adv_code::lib::grid_utils::{Coordinates, Direction, GridCoordinates};
use adv_code::year2024::day04::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use strum::IntoEnumIterator;

const INPUT_FILE: &str = "input/2024/04/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 04, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 2344);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let xmas = vec![Some(&'X'), Some(&'M'), Some(&'A'), Some(&'S')];
    let grid = parse_input(&file_input_path);

    grid.indexed_iter()
        .map(|(coordinates, _)| GridCoordinates::from(coordinates))
        .map(|coordinates| {
            Direction::iter()
                .map(|direction|
                    (0..4).map(|n| coordinates.move_to_n(direction, n))
                        .map(|c| c.map(|GridCoordinates(row, col)| grid.get(row, col)).flatten())
                        .collect()
                )
                .filter(|letters: &Vec<Option<&char>>| letters.eq(&xmas))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1;

    const TEST_INPUT_FILE: &str = "input/2024/04/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 18);
    }
}