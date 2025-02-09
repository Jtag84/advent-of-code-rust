use crate::lib::parser_commons::{number, read_file_to_string};
use nom::character::complete::{multispace0, multispace1};
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

fn line(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (num1, _, num2, _)) = tuple((number, multispace1, number, multispace0))(input)?;
    Ok((input, (num1, num2)))
}

fn parse_columns(input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    let (input, pairs) = many0(line)(input)?;

    let (left, right): (Vec<_>, Vec<_>) = pairs.into_iter().map(|(l, r)| (l, r)).unzip();

    Ok((input, (left, right)))
}

pub type ParsedInput = (Vec<i32>, Vec<i32>);

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let parsed_columns_result = parse_columns(&file_string);
    parsed_columns_result.expect("Parsing error").1
}
