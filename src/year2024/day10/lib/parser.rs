use crate::lib::parser_commons::parse_grid_digit;
use grid::Grid;

pub fn parse_input(input_path: &str) -> Grid<u32> {
    parse_grid_digit(input_path)
}
