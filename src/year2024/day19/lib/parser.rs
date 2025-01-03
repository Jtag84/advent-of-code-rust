use crate::lib::parser_commons::read_file_to_string;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn patterns_parser(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn designs_parser(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(newline, alpha1)(input)
}

pub fn parse_input(input_path: &str) -> (Vec<String>, Vec<String>) {
    let file_string = read_file_to_string(input_path);

    let (_, result) = separated_pair(patterns_parser, multispace1, designs_parser)(&file_string)
        .expect("Parse error");

    (
        result.0.into_iter().map(String::from).collect(),
        result.1.into_iter().map(String::from).collect(),
    )
}
