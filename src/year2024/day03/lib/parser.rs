use crate::lib::parser_commons::{find_all, number, read_file_to_string};
use crate::year2024::day03::lib::parser::OperationInstruction::{Dont, Mul};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::tuple;
use nom::IResult;
use OperationInstruction::Do;

#[derive(Debug, Clone)]
pub enum OperationInstruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn mul_parser(input: &str) -> IResult<&str, OperationInstruction> {
    let (input, (_, num1, _, num2, _)) =
        tuple((tag("mul("), number, char(','), number, char(')')))(input)?;
    Ok((input, Mul(num1, num2)))
}

fn do_parser(input: &str) -> IResult<&str, OperationInstruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Do))
}

fn dont_parser(input: &str) -> IResult<&str, OperationInstruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Dont))
}

fn operation_instruction_parser(input: &str) -> IResult<&str, OperationInstruction> {
    alt((mul_parser, do_parser, dont_parser))(input)
}

fn find_all_operation_instructions(input: &str) -> Vec<OperationInstruction> {
    find_all(input, operation_instruction_parser)
}

pub type ParsedInput = Vec<OperationInstruction>;

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);
    find_all_operation_instructions(&file_string)
}
