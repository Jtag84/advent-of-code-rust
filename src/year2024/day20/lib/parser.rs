use crate::lib::grid_utils::{set_grid_element, GridCoordinates};
use crate::lib::parser_commons::parse_grid;
use grid::Grid;

pub type Racetrack = Grid<char>;
pub type Start = GridCoordinates;
pub type End = GridCoordinates;

pub fn parse_input(input_path: &str) -> (Racetrack, Start, End) {
    let mut racetrack = parse_grid(&input_path);
    let start = racetrack
        .indexed_iter()
        .find(|(_, v)| **v == 'S')
        .map(|(c, _)| GridCoordinates::from(c))
        .unwrap();
    let end = racetrack
        .indexed_iter()
        .find(|(_, v)| **v == 'E')
        .map(|(c, _)| GridCoordinates::from(c))
        .unwrap();

    set_grid_element(&mut racetrack, &start, '.');
    set_grid_element(&mut racetrack, &end, '.');

    (racetrack, start, end)
}
