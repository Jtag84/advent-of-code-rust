use crate::lib::grid_utils::Direction::{East, North, South, West};
use crate::lib::grid_utils::{set_grid_element, Coordinates, Direction, GridCoordinates};
use crate::year2024::day15::lib::parser::{ParsedInput, RobotPosition};
use grid::Grid;
use itertools::Itertools;

pub fn part2((grid, robot_position, movements): ParsedInput) -> String {
    let mut resized_grid = resizing_grid(grid);
    let new_robot_position = GridCoordinates(robot_position.row(), robot_position.column() * 2);

    // println!(
    //     "old position {:?}, New robot position is {:?}",
    //     robot_position, new_robot_position
    // );
    // println!("{}", grid_to_str(&resized_grid));

    let final_robot_position =
        movements
            .iter()
            .fold(new_robot_position, |current_robot_position, movement| {
                move_robot(&mut resized_grid, current_robot_position, *movement)
            });
    set_grid_element(&mut resized_grid, &final_robot_position, '@');

    // println!("final robot position is {:?}", final_robot_position);
    // println!("{}", grid_to_str(&resized_grid));

    resized_grid
        .indexed_iter()
        .filter(|(_, grid_value)| **grid_value == '[')
        .map(|(box_coordinates, _)| box_coordinates.0 * 100 + box_coordinates.1)
        .sum::<usize>()
        .to_string()
}

fn resizing_grid(grid: Grid<char>) -> Grid<char> {
    let expanded_rows: Vec<Vec<char>> = grid
        .iter_rows()
        .map(|row| {
            row.into_iter()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    '.' | '@' => vec!['.', '.'],
                    'O' => vec!['[', ']'],
                    _ => panic!("Invalid char: {}", c),
                })
                .collect()
        })
        .collect();
    Grid::from(expanded_rows)
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
        '[' | ']' => {
            let connected_blocks = if direction == East || direction == West {
                vec![new_position]
            } else {
                expand_block((new_position, *grid_value))
                    .into_iter()
                    .map(|(c, _)| c)
                    .collect()
            };
            if push_block(grid, connected_blocks, direction) {
                new_position
            } else {
                robot_position
            }
        }
        _ => panic!("Wrong value"),
    }
}

fn push_block(
    grid: &mut Grid<char>,
    connected_block_parts_coordinates: Vec<GridCoordinates>,
    direction: Direction,
) -> bool {
    let new_coordinates: Vec<_> = connected_block_parts_coordinates
        .iter()
        .filter_map(|current| current.move_to(direction).map(|new| (*current, new)))
        .collect();

    let grid_values: Vec<_> = new_coordinates
        .iter()
        .filter_map(|(_, new)| grid.get(new.row(), new.column()).map(|v| (*new, *v)))
        .collect();

    if grid_values.iter().any(|(_, v)| *v == '#') {
        return false;
    }

    if grid_values.iter().all(|(_, v)| *v == '.') {
        swap_grid_elements(grid, new_coordinates);
        return true;
    }

    let trimmed_grid_values = if direction == North || direction == South {
        grid_values
            .into_iter()
            .filter(|(_, v)| *v != '.')
            .flat_map(expand_block)
            .unique_by(|(c, _)| *c)
            .collect()
    } else {
        grid_values
    };

    if push_block(
        grid,
        trimmed_grid_values.iter().map(|(c, _)| *c).collect(),
        direction,
    ) {
        swap_grid_elements(grid, new_coordinates);
        true
    } else {
        false
    }
}

fn expand_block(
    block @ (coordinates, grid_element): (GridCoordinates, char),
) -> Vec<(GridCoordinates, char)> {
    let matching_block = match grid_element {
        ']' => (coordinates.west().unwrap(), '['),
        '[' => (coordinates.east().unwrap(), ']'),
        _ => panic!("Invalid grid element"),
    };
    vec![matching_block, block]
}

fn swap_grid_elements(
    grid: &mut Grid<char>,
    new_coordinates: Vec<(GridCoordinates, GridCoordinates)>,
) {
    new_coordinates
        .iter()
        .for_each(|(current, new)| grid.swap((*current).into(), (*new).into()));
}

#[cfg(test)]
mod test {
    use crate::year2024::day15::lib::part2::part2;
    use crate::year2024::day15::lib::YEAR_2024_DAY_15_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_15_SOLUTION.get_parsed_test_inputs(2)),
            "9021"
        );
    }
}
