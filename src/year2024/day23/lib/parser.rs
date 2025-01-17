use crate::lib::parser_commons::read_file_to_string;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};

pub type ParsedInput = HashMap<Computer, HashSet<Computer>>;
pub type Computer = String;
pub type Connection = (Computer, Computer);
pub type Connections = Vec<Connection>;

fn parse_connection(input: &str) -> IResult<&str, Connections> {
    let (input, (computer1, computer2)) = separated_pair(alpha1, tag("-"), alpha1)(input)?;
    Ok((
        input,
        vec![
            (computer1.to_string(), computer2.to_string()),
            (computer2.to_string(), computer1.to_string()),
        ],
    ))
}

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let (_, connections_vec) =
        separated_list1(newline, parse_connection)(&file_string).expect("Parse error");

    connections_vec
        .into_iter()
        .flatten()
        .into_group_map()
        .into_iter()
        .map(|(computer, vec)| (computer, HashSet::from_iter(vec)))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::year2024::day23::lib::YEAR_2024_DAY_23_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = YEAR_2024_DAY_23_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = YEAR_2024_DAY_23_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
