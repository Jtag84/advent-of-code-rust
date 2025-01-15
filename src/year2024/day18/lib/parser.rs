use crate::lib::grid_utils::XYCoordinates;
use crate::lib::parser_commons::{number, read_file_to_string};
use nom::character::complete::{char, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

pub type ParsedInput = Vec<XYCoordinates>;

fn xy_coordinate_parser(input: &str) -> IResult<&str, XYCoordinates> {
    let (input, xy) = separated_pair(number, char(','), number)(input)?;
    Ok((input, XYCoordinates::from(xy)))
}

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let parsed_coordinates = separated_list1(newline, xy_coordinate_parser)(&file_string);
    parsed_coordinates.expect("Parsing error").1
}
