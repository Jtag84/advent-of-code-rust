use crate::lib::parser_commons::parse_grid;
use grid::Grid;

pub fn parse_input(input_path: &str) -> Grid<char> {
    parse_grid(input_path)
}
