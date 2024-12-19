use crate::lib::grid_utils::Direction::North;
use crate::lib::grid_utils::{Direction, GridCoordinates};
use crate::lib::parser_commons::parse_grid;
use grid::Grid;

pub type GuardPosition = (GridCoordinates, Direction);
pub fn parse_input(input_path: &str) -> (GuardPosition, Grid<char>) {
    let mut grid = parse_grid(&input_path);

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
