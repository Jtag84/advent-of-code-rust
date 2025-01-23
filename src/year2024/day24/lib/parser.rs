use crate::lib::parser_commons::read_file_to_string;
use crate::year2024::day24::lib::parser::WireState::{AND, ONE, OR, XOR, ZERO};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace1, newline};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;

pub type WireName = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WireState {
    AND(WireName, WireName),
    OR(WireName, WireName),
    XOR(WireName, WireName),
    ONE,
    ZERO,
}

impl WireState {
    pub fn is_xor(&self) -> bool {
        match self {
            XOR(_, _) => true,
            _ => false,
        }
    }

    pub fn is_and(&self) -> bool {
        match self {
            AND(_, _) => true,
            _ => false,
        }
    }

    pub fn is_or(&self) -> bool {
        match self {
            OR(_, _) => true,
            _ => false,
        }
    }

    pub fn has_xy_inputs(&self) -> bool {
        match self {
            AND(left, right) | OR(left, right) | XOR(left, right) => {
                left.starts_with("x") && right.starts_with("y")
                    || left.starts_with("y") && right.starts_with("x")
            }
            _ => false,
        }
    }

    pub fn has_initial_xy_inputs(&self) -> bool {
        match self {
            AND(left, right) | XOR(left, right) => {
                left == "x00" && right == "y00" || left == "y00" && right == "x00"
            }
            _ => false,
        }
    }

    pub fn is_known_value(&self) -> bool {
        match self {
            ONE | ZERO => true,
            _ => false,
        }
    }
}

pub type ParsedInput = HashMap<WireName, WireState>;

fn parse_initial_wire_values(input: &str) -> IResult<&str, Vec<(WireName, WireState)>> {
    separated_list1(newline, parse_initial_wire_value)(input)
}

fn parse_initial_wire_value(input: &str) -> IResult<&str, (WireName, WireState)> {
    let (input, (wire_name, state)) = separated_pair(alphanumeric1, tag(": "), digit1)(input)?;
    let wire_state = match state {
        "0" => ZERO,
        "1" => ONE,
        _ => panic!("Wrong state {}", state),
    };

    Ok((input, (wire_name.to_string(), wire_state)))
}

fn parse_wire_operations(input: &str) -> IResult<&str, Vec<(WireName, WireState)>> {
    separated_list1(newline, parse_wire_operation)(input)
}

fn parse_wire_operation(input: &str) -> IResult<&str, (WireName, WireState)> {
    let (input, (left_wire_name, _, operation, _, right_wire_name, _, result_wire_name)) =
        tuple((
            alphanumeric1,
            tag(" "),
            alpha1,
            tag(" "),
            alphanumeric1,
            tag(" -> "),
            alphanumeric1,
        ))(input)?;

    let wire_state = match operation {
        "AND" => AND(left_wire_name.to_string(), right_wire_name.to_string()),
        "OR" => OR(left_wire_name.to_string(), right_wire_name.to_string()),
        "XOR" => XOR(left_wire_name.to_string(), right_wire_name.to_string()),
        _ => panic!("Wrong operation {}", operation),
    };

    Ok((input, (result_wire_name.to_string(), wire_state)))
}

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let (_, (wire_name_states_top, wire_name_states_bottom)) = tuple((
        parse_initial_wire_values,
        preceded(multispace1, parse_wire_operations),
    ))(&file_string)
    .expect("Error parsing input file");

    vec![wire_name_states_top, wire_name_states_bottom]
        .into_iter()
        .flatten()
        .into_grouping_map()
        .aggregate(|_acc, _key, val| Some(val))
}

#[cfg(test)]
mod test {
    use crate::year2024::day24::lib::YEAR_2024_DAY_24_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = YEAR_2024_DAY_24_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = YEAR_2024_DAY_24_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
