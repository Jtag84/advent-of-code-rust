use adv_code::lib::grid_utils::{Coordinates, GridCoordinates};
use adv_code::year2024::day20::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/2024/20/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 20, 1);

    let result = time_snippet!(part1(INPUT_FILE, 100));
    println!("Result = {}", result);

    assert_eq!(result, 1343);

    Ok(())
}

fn part1(file_input_path: &str, cheat_cost_saving: usize) -> usize {
    let (racetrack, start, end) = parse_input(&file_input_path);
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
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day20::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/20/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE, 12), 8);
    }

    #[test]
    fn test_parser() {
        let (racetrack, start, end) = parse_input(TEST_INPUT_FILE);
        println!("start: {start:?}, end: {end:?}");
        println!("{}", grid_to_str(&racetrack));
    }
}
