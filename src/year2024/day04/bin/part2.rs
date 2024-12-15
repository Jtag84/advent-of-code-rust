use adv_code::lib::grid_utils::{Coordinates, GridCoordinates};
use adv_code::year2024::day04::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/04/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 04, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1815);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let mas = vec![Some(&'M'), Some(&'A'), Some(&'S')];
    let sam = vec![Some(&'S'), Some(&'A'), Some(&'M')];
    let grid = parse_input(&file_input_path);

    grid.indexed_iter()
        .map(|(coordinates, _)| GridCoordinates::from(coordinates))
        .map(|coordinates| {
            vec![
                vec![
                    coordinates.north_west(),
                    Some(coordinates),
                    coordinates.south_east(),
                ],
                vec![
                    coordinates.north_east(),
                    Some(coordinates),
                    coordinates.south_west(),
                ],
            ]
        })
        .map(|coords| {
            coords
                .into_iter()
                .map(|coords: Vec<Option<GridCoordinates>>| {
                    coords
                        .iter()
                        .map(|optional_coordinate| {
                            optional_coordinate
                                .map(|c| grid.get(c.row(), c.column()))
                                .flatten()
                        })
                        .collect()
                })
                .collect()
        })
        .filter(|crossed_letters: &Vec<Vec<Option<&char>>>| {
            let first_word = crossed_letters.get(0).unwrap();
            let second_word = crossed_letters.get(1).unwrap();
            (first_word.eq(&sam) || first_word.eq(&mas))
                && (second_word.eq(&sam) || second_word.eq(&mas))
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::part2;

    const TEST_INPUT_FILE: &str = "input/2024/04/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 9);
    }
}
