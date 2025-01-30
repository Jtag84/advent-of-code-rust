use crate::lib::parser_commons::{number, read_file_to_string};
use crate::year2016::day12::lib::parser::Instruction::{Copy, Dec, Inc, Jnz};
use crate::year2016::day12::lib::parser::Register::{A, B, C, D};
use itertools::Either;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::streaming::{newline, one_of};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use strum_macros::EnumTable;

pub type ParsedInput = Vec<Instruction>;

pub type Value = isize;

#[derive(Clone, Debug, EnumTable, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Copy(Either<Value, Register>, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Either<Value, Register>, Value),
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    let (input, register) = one_of("abcd")(input)?;
    let register = match register {
        'a' => A,
        'b' => B,
        'c' => C,
        'd' => D,
        _ => unreachable!(),
    };
    Ok((input, register))
}

fn parse_either_value_register(input: &str) -> IResult<&str, Either<Value, Register>> {
    let (input, either_value_register) = alt((
        map(number, Either::Left),
        map(parse_register, Either::Right),
    ))(input)?;
    Ok((input, either_value_register))
}

fn parse_copy(input: &str) -> IResult<&str, Instruction> {
    let (input, (either_value_register, register)) = preceded(
        tag("cpy "),
        separated_pair(parse_either_value_register, tag(" "), parse_register),
    )(input)?;
    Ok((input, Copy(either_value_register, register)))
}

fn parse_inc(input: &str) -> IResult<&str, Instruction> {
    let (input, register) = preceded(tag("inc "), parse_register)(input)?;
    Ok((input, Inc(register)))
}

fn parse_dec(input: &str) -> IResult<&str, Instruction> {
    let (input, register) = preceded(tag("dec "), parse_register)(input)?;
    Ok((input, Dec(register)))
}

fn parse_jnz(input: &str) -> IResult<&str, Instruction> {
    let (input, (either_value_register, value)) = preceded(
        tag("jnz "),
        separated_pair(parse_either_value_register, tag(" "), number),
    )(input)?;
    Ok((input, Jnz(either_value_register, value)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = alt((parse_copy, parse_dec, parse_inc, parse_jnz))(input)?;
    Ok((input, instruction))
}

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let (_, instructions) =
        separated_list1(newline, parse_instruction)(&file_string).expect("Parse error");
    instructions
}

#[cfg(test)]
mod test {
    use crate::year2016::day12::lib::YEAR_2016_DAY_12_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = YEAR_2016_DAY_12_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = YEAR_2016_DAY_12_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
