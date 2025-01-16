use crate::lib::parser_commons::{number, read_file_to_string};
use nom::character::complete::{multispace1, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn report(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, report) = separated_list1(space1, number)(input)?;
    Ok((input, report))
}

fn reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, reports) = separated_list1(multispace1, report)(input)?;

    Ok((input, reports))
}

pub type ParsedInput = Vec<Vec<i32>>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let parsed_reports_result = reports(&file_string);
    parsed_reports_result.expect("Parsing error").1
}
