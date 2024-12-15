use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use OperationInstruction::Do;
use crate::lib::parser_commons::{find_all, read_file_to_string};
use crate::year2024::day03::parser::OperationInstruction::{Dont, Mul};

#[derive(Debug)]
pub enum OperationInstruction {
    Mul(i32, i32),
    Do,
    Dont
}

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn mul_parser(input: &str) -> IResult<&str, OperationInstruction> {
    let (input, (_, num1, _, num2, _)) = tuple((
        tag("mul("),
        number,
        char(','),
        number,
        char(')'),
    ))(input)?;
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

fn find_all_muls(input: &str) -> Vec<(i32, i32)> {
    find_all(input, mul_parser).iter().map(|operation| {
        match operation {
            Mul(x, y) => (*x, *y),
            _ => panic!()
        }
    }).collect()
}

pub fn part1_parse_input(input_path: &str) -> Vec<(i32, i32)> {
    let file_string = read_file_to_string(input_path);
    find_all_muls(&file_string)
}

pub fn part2_parse_input(input_path: &str) -> Vec<OperationInstruction> {
    let file_string = read_file_to_string(input_path);
    find_all_operation_instructions(&file_string)
}