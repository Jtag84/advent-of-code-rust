use adv_code::lib::grid_utils::{Coordinates, Direction, GridCoordinates};
use adv_code::year2024::day15::lib::parser::{parse_input, RobotPosition};
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use grid::Grid;

const INPUT_FILE: &str = "input/2024/15/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 15, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1563092);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let (mut grid, robot_position, movements) = parse_input(&file_input_path);

    movements
        .iter()
        .fold(robot_position, |current_robot_position, movement| {
            move_robot(&mut grid, current_robot_position, *movement)
        });

    grid.indexed_iter()
        .filter(|(_, grid_value)| **grid_value == 'O')
        .map(|(box_coordinates, _)| box_coordinates.0 * 100 + box_coordinates.1)
        .sum()
}

fn move_robot(
    grid: &mut Grid<char>,
    robot_position: RobotPosition,
    direction: Direction,
) -> RobotPosition {
    let Some(new_position) = robot_position.move_to(direction) else {
        panic!("Wrong robot position")
    };
    let Some(grid_value) = grid.get(new_position.row(), new_position.column()) else {
        panic!("Wrong grid value")
    };
    match grid_value {
        '.' => new_position,
        '#' => robot_position,
        'O' => {
            let empty_position_option =
                find_next_empty_in_direction(grid, robot_position, direction);
            match empty_position_option {
                Some(empty_position) => {
                    grid.swap(empty_position.into(), new_position.into());
                    new_position
                }
                None => robot_position,
            }
        }
        _ => panic!("Wrong value"),
    }
}

fn find_next_empty_in_direction(
    grid: &Grid<char>,
    from: GridCoordinates,
    direction: Direction,
) -> Option<GridCoordinates> {
    let new_position = from.move_to(direction)?;

    let new_grid_value = grid.get(new_position.row(), new_position.column())?;
    match new_grid_value {
        '.' => Some(new_position),
        '#' => None,
        'O' => find_next_empty_in_direction(grid, new_position, direction),
        _ => panic!("Wrong grid value"),
    }
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day15::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/15/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 10092);
    }

    #[test]
    fn test_parser() {
        let (grid, robot_position, movements) = parse_input(TEST_INPUT_FILE);
        println!("{}", grid_to_str(&grid));
        println!("robot_position = {robot_position:?}, movements = {movements:?}",);
    }
}
