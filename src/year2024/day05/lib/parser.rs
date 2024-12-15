use std::collections::HashSet;
use crate::lib::parser_commons::{number, read_file_to_string};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

pub type Rule = (usize, usize);

fn rule_parser(input: &str) -> IResult<&str, Rule> {
    let (input, (rule_left, _, rule_right, _)) = tuple((number, tag("|"), number, newline))(input)?;
    Ok((input, (rule_left,rule_right)))
}

fn rules_parser(input: &str) -> IResult<&str, HashSet<Rule>> {
    let (input, rules) = many1(rule_parser)(input)?;
    Ok((input, rules.into_iter().collect()))
}

pub type Update = usize;
pub type UpdateLine = Vec<Update>;

fn update_parser(input: &str) -> IResult<&str, Vec<Update>> {
    let (input, updates) = separated_list1(tag(","), number)(input)?;
    Ok((input, updates))
}

fn updates_parser(input: &str) -> IResult<&str, Vec<Vec<Update>>> {
    let (input, updates_list) = separated_list1(newline, update_parser)(input)?;
    Ok((input, updates_list))
}

pub fn parse_input(input_path: &str) -> (HashSet<Rule>, Vec<UpdateLine>) {
    let file_string = read_file_to_string(input_path);

    let (_, (rules, _, updates)) = tuple((rules_parser, newline, updates_parser))(&file_string).expect("Parsing error");

    (rules, updates)
}