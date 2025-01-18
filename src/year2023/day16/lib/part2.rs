use crate::lib::grid_utils::Direction::{East, North, South, West};
use crate::lib::grid_utils::{GridCoordinates, Position};
use crate::year2023::day16::lib::moving_beam;
use crate::year2023::day16::lib::parser::ParsedInput;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::collections::HashSet;

pub fn part2(grid: ParsedInput) -> String {
    let (max_row, max_col) = grid.size();
    let top_and_bottom_start_positions: Vec<Position<GridCoordinates>> = (0isize..max_col as isize)
        .flat_map(|col| {
            vec![
                Position(GridCoordinates(0, col), South),
                Position(GridCoordinates(max_row as isize - 1, col), North),
            ]
        })
        .collect();

    let left_and_right_start_positions = (0isize..max_row as isize)
        .flat_map(|row| {
            vec![
                Position(GridCoordinates(row, 0), East),
                Position(GridCoordinates(row, max_col as isize - 1), West),
            ]
        })
        .collect();

    vec![
        top_and_bottom_start_positions,
        left_and_right_start_positions,
    ]
    .into_par_iter()
    .flatten()
    .map(|start_position| {
        moving_beam(&grid, start_position, &mut HashSet::new())
            .iter()
            .map(|position| position.coordinates())
            .unique()
            .count()
    })
    .max()
    .unwrap()
    .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2023::day16::lib::part2::part2;
    use crate::year2023::day16::lib::YEAR_2023_DAY_16_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2023_DAY_16_SOLUTION.get_parsed_test_inputs(2)),
            "51"
        );
    }
}
