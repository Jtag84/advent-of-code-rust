use adv_code::lib::grid_utils::{Coordinates, Direction, Position};
use adv_code::year2024::day16::lib::parser::parse_input;
use adv_code::year2024::day16::lib::successors;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use pathfinding::prelude::astar_bag_collect;

const INPUT_FILE: &str = "input/2024/16/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 16, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 645);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let (grid, start, end) = parse_input(&file_input_path);

    let start_position = Position(start, Direction::East);
    let shortest_paths = astar_bag_collect(
        &start_position,
        |p| successors(&grid, p),
        |p| p.coordinates().manhattan_distance(end),
        |p| p.coordinates() == end,
    )
    .unwrap();
    shortest_paths
        .0
        .into_iter()
        .flatten()
        .map(|p| p.coordinates())
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day16::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/16/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 64);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
