use crate::lib::parser_commons::read_file_to_string;
use grid::Grid;

pub fn parse_input(input_path: &str) -> Grid<u32> {
    let file_string = read_file_to_string(input_path);

    let parsed_input: Vec<Vec<u32>> = file_string
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Grid::from(&parsed_input)
}
