use crate::lib::parser_commons::parse_grid;
use grid::Grid;

pub type ParsedInput = Grid<char>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    parse_grid(input_path)
}
