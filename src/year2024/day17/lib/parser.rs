use crate::lib::parser_commons::{number, read_file_to_string};
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, one_of};
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::IResult;

pub type Register = isize;
pub type RegisterA = Register;
pub type RegisterB = Register;
pub type RegisterC = Register;
pub type Program = Vec<isize>;

fn register_parser(input: &str) -> IResult<&str, isize> {
    let (input, (_, _, _, register_value)) =
        tuple((tag("Register "), one_of("ABC"), tag(": "), number))(input)?;
    Ok((input, register_value))
}

fn program_parser(input: &str) -> IResult<&str, Program> {
    let (input, (_, program)) =
        tuple((tag("Program: "), separated_list1(char(','), number)))(input)?;

    Ok((input, program))
}

pub fn parse_input(input_path: &str) -> (RegisterA, RegisterB, RegisterC, Program) {
    let file_string = read_file_to_string(input_path);

    let result = tuple((
        terminated(register_parser, newline),
        terminated(register_parser, newline),
        terminated(register_parser, tuple((newline, newline))),
        program_parser,
    ))(&file_string)
    .expect("Parsing error")
    .1;
    result
}
