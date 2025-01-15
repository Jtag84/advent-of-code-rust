use crate::lib::parser_commons::{number, read_file_to_string};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

type Result = isize;
type Numbers = Vec<isize>;
pub type Equation = (Result, Numbers);

fn equation_parser(input: &str) -> IResult<&str, Equation> {
    let (input, (result, _, numbers)) =
        tuple((number, tag(": "), separated_list1(tag(" "), number)))(input)?;

    Ok((input, (result, numbers)))
}

pub type ParsedInput = Vec<Equation>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let parsed_equations_result = separated_list1(newline, equation_parser)(&file_string);
    parsed_equations_result.expect("Parsing error").1
}
