use crate::lib::parser_commons::{number, read_file_to_string};
use nom::character::complete::space1;
use nom::multi::separated_list1;

pub type Stone = usize;

pub type ParsedInput = Vec<Stone>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let (_, stones) = separated_list1(space1, number)(&file_string).expect("Parse error");
    stones
}
