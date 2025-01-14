use crate::lib::grid_utils::{Coordinates, GridCoordinates};
use crate::year2024::day20::lib::parser::ParsedInput;
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashMap;

pub fn part1((racetrack, start, end): ParsedInput, cheat_cost_saving: usize) -> String {
    let (no_cheat_path, _) = astar(
        &start,
        |node| {
            node.cardinals().into_iter().filter_map(|c| {
                racetrack
                    .get(c.row(), c.column())
                    .filter(|v| **v == '.')
                    .map(|_| (c, 1))
            })
        },
        |node| node.manhattan_distance(end),
        |node| *node == end,
    )
    .unwrap();

    let walls_around_path: Vec<GridCoordinates> = no_cheat_path
        .clone()
        .iter()
        .flat_map(|c| c.cardinals())
        .unique()
        .filter(|c| {
            c.get_grid_element(&racetrack)
                .filter(|e| **e == '#')
                .is_some()
        })
        .collect();

    let coordinate_path_index_map: HashMap<GridCoordinates, usize> = no_cheat_path
        .into_iter()
        .enumerate()
        .map(|(index, coordinate)| (coordinate, index))
        .collect();

    walls_around_path
        .iter()
        .filter_map(|wall_coordinates| {
            wall_coordinates
                .cardinals()
                .iter()
                .filter(|c| {
                    c.get_grid_element(&racetrack)
                        .filter(|e| **e == '.')
                        .is_some()
                })
                .tuple_combinations::<(_, _)>()
                .map(|(c1, c2)| {
                    (*coordinate_path_index_map.get(c1).unwrap() as isize
                        - *coordinate_path_index_map.get(c2).unwrap() as isize)
                        .abs()
                        - 2
                })
                .max()
        })
        .filter(|cost_saving| *cost_saving >= cheat_cost_saving as isize)
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day20::lib::part1::part1;
    use crate::year2024::day20::lib::YEAR_2024_DAY_20_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_20_SOLUTION.get_parsed_test_inputs(1), 12),
            "8"
        );
    }
}
