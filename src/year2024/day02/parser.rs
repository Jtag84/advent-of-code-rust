use crate::read_file_to_string;
use nom::character::complete::{digit1, multispace1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn report(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, report) = separated_list1(space1, number)(input)?;
    Ok((input, report))
}

fn reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, reports) = separated_list1(multispace1,report)(input)?;

    Ok((input, reports))
}

pub fn parse_input(input_path: &str) -> Vec<Vec<i32>> {
    let file_string = read_file_to_string(input_path);

    let parsed_reports_result = reports(&file_string);
    parsed_reports_result.expect("Parsing error").1
}