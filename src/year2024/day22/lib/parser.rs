use crate::lib::parser_commons::{number, read_file_to_string};
use nom::character::complete::newline;
use nom::multi::separated_list1;

pub fn parse_input(input_path: &str) -> Vec<usize> {
    let file_string = read_file_to_string(input_path);

    let parsed_columns_result = separated_list1(newline, number)(&file_string);
    parsed_columns_result.expect("Parsing error").1
}
