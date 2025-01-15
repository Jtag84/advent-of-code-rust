use crate::lib::grid_utils::{Coordinates, Direction, GridCoordinates};
use crate::year2024::day15::lib::parser::{ParsedInput, RobotPosition};
use grid::Grid;

pub fn part1((mut grid, robot_position, movements): ParsedInput) -> String {
    movements
        .iter()
        .fold(robot_position, |current_robot_position, movement| {
            move_robot(&mut grid, current_robot_position, *movement)
        });

    grid.indexed_iter()
        .filter(|(_, grid_value)| **grid_value == 'O')
        .map(|(box_coordinates, _)| box_coordinates.0 * 100 + box_coordinates.1)
        .sum::<usize>()
        .to_string()
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
    use crate::year2024::day15::lib::part1::part1;
    use crate::year2024::day15::lib::YEAR_2024_DAY_15_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_15_SOLUTION.get_parsed_test_inputs(1)),
            "10092"
        );
    }
}
