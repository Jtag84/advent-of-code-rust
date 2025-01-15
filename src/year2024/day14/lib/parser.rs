use crate::lib::grid_utils::XYCoordinates;
use crate::lib::parser_commons::{number, read_file_to_string};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

pub type Speed = XYCoordinates;
pub type PositionAndSpeed = (XYCoordinates, Speed);

fn position_and_speed_parser(input: &str) -> IResult<&str, PositionAndSpeed> {
    let (input, (_, px, _, py, _, sx, _, sy)) = tuple((
        tag("p="),
        number,
        tag(","),
        number,
        tag(" v="),
        number,
        tag(","),
        number,
    ))(input)?;
    Ok((input, (XYCoordinates(px, py), XYCoordinates(sx, sy))))
}

pub type ParsedInput = Vec<PositionAndSpeed>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let parsed = separated_list1(newline, position_and_speed_parser)(&file_string);
    parsed.expect("Parsing error").1
}
