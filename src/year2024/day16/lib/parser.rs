use crate::lib::grid_utils::{set_grid_element, GridCoordinates};
use crate::lib::parser_commons::parse_grid;
use grid::Grid;

type Start = GridCoordinates;
type End = GridCoordinates;

pub fn parse_input(input_path: &str) -> (Grid<char>, Start, End) {
    let mut grid = parse_grid(input_path);

    let start = grid
        .indexed_iter()
        .find(|(_, v)| **v == 'S')
        .map(|(c, _)| GridCoordinates::from(c))
        .unwrap();

    let end = grid
        .indexed_iter()
        .find(|(_, v)| **v == 'E')
        .map(|(c, _)| GridCoordinates::from(c))
        .unwrap();

    set_grid_element(&mut grid, &start, '.');
    set_grid_element(&mut grid, &end, '.');

    (grid, start, end)
}
