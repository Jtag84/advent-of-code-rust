use crate::lib::grid_utils::{Coordinates, GridCoordinates};
use crate::year2024::day04::lib::parser::ParsedInput;

pub fn part2(grid: ParsedInput) -> String {
    let mas = vec![Some(&'M'), Some(&'A'), Some(&'S')];
    let sam = vec![Some(&'S'), Some(&'A'), Some(&'M')];

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
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day04::lib::{part2, YEAR_2024_DAY_04_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_04_SOLUTION.get_parsed_test_inputs(2)),
            "9"
        );
    }
}
