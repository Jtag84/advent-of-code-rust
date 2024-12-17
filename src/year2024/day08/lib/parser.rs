use crate::lib::parser_commons::read_file_to_string;
use grid::Grid;

pub fn parse_input(input_path: &str) -> Grid<char> {
    let file_string = read_file_to_string(input_path);

    let parsed_input: Vec<Vec<char>> = file_string.lines().map(|l| l.chars().collect()).collect();
    Grid::from(&parsed_input)
}
