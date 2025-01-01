use adv_code::lib::grid_utils::{Coordinates, Direction, Position};
use adv_code::year2024::day16::lib::parser::parse_input;
use adv_code::year2024::day16::lib::successors;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use pathfinding::prelude::astar;

const INPUT_FILE: &str = "input/2024/16/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 16, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 143580);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let (grid, start, end) = parse_input(&file_input_path);

    let start_position = Position(start, Direction::East);
    let shortest_path = astar(
        &start_position,
        |p| successors(&grid, p),
        |p| p.coordinates().manhattan_distance(end),
        |p| p.coordinates() == end,
    )
    .unwrap();
    shortest_path.1
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day16::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/16/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 11048);
    }

    #[test]
    fn test_parser() {
        let (grid, start, end) = parse_input(TEST_INPUT_FILE);
        println!("{}", grid_to_str(&grid));
        println!("start: {start:?}, end: {end:?}");
    }
}
