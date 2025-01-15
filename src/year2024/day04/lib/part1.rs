use crate::lib::grid_utils::{Coordinates, Direction, GridCoordinates};
use crate::year2024::day04::lib::parser::ParsedInput;
use strum::IntoEnumIterator;

pub fn part1(grid: ParsedInput) -> String {
    let xmas = vec![Some(&'X'), Some(&'M'), Some(&'A'), Some(&'S')];

    grid.indexed_iter()
        .map(|(coordinates, _)| GridCoordinates::from(coordinates))
        .map(|coordinates| {
            Direction::iter()
                .map(|direction| {
                    (0..4)
                        .map(|n| coordinates.move_to_n(direction, n))
                        .map(|c| {
                            c.map(|GridCoordinates(row, col)| grid.get(row, col))
                                .flatten()
                        })
                        .collect()
                })
                .filter(|letters: &Vec<Option<&char>>| letters.eq(&xmas))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day04::lib::{part1, YEAR_2024_DAY_04_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_04_SOLUTION.get_parsed_test_inputs(1)),
            "18"
        );
    }
}
