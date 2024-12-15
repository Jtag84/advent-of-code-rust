use crate::lib::grid_utils::Direction::North;
use crate::lib::grid_utils::{Direction, GridCoordinates};
use crate::lib::parser_commons::read_file_to_string;
use grid::Grid;

pub type GuardPosition = (GridCoordinates, Direction);
pub fn parse_input(input_path: &str) -> (GuardPosition, Grid<char>) {
    let file_string = read_file_to_string(input_path);

    let parsed_input: Vec<Vec<char>> = file_string.lines().map(|l| l.chars().collect()).collect();
    let mut grid = Grid::from(&parsed_input);

    let guard_start_coordinates: GridCoordinates = grid
        .indexed_iter()
        .find(|(_, map_element)| **map_element == '^')
        .unwrap()
        .0
        .try_into()
        .expect("can't convert to GridCoordinates");

    // delete the guard position
    *(grid
        .get_mut(
            guard_start_coordinates.row(),
            guard_start_coordinates.column(),
        )
        .unwrap()) = '.';

    ((guard_start_coordinates, North), grid)
}
