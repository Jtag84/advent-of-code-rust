use crate::lib::parser_commons::parse_grid_digit;
use grid::Grid;

pub type ParsedInput = Grid<u32>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    parse_grid_digit(input_path)
}
