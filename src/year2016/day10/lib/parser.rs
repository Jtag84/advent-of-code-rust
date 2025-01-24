use crate::lib::parser_commons::{number, read_file_to_string};
use crate::year2016::day10::lib::parser::Entry::{BotRuleEntry, InitialValueEntry};
use crate::year2016::day10::lib::parser::ValueDestination::{ToBot, ToOutputBin};
use itertools::{Either, Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::HashMap;

pub type Bots = HashMap<BotId, Bot>;
pub type OutputBins = HashMap<OutputBinId, OutputBin>;

pub type BotId = usize;
pub type Value = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct Bot {
    pub value: Option<Value>,
    pub gives_low_to: ValueDestination,
    pub gives_high_to: ValueDestination,
}

pub type OutputBinId = usize;
#[derive(Debug, PartialEq, Clone)]
pub struct OutputBin {
    pub values: Vec<Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueDestination {
    ToBot(BotId),
    ToOutputBin(OutputBinId),
}

pub type InitialValue = (BotId, Value);
pub type InitialValues = Vec<InitialValue>;
pub type ParsedInput = (Bots, OutputBins, InitialValues);

pub enum Entry {
    InitialValueEntry(InitialValue),
    BotRuleEntry((BotId, Bot)),
}

fn initial_value_parser(input: &str) -> IResult<&str, InitialValue> {
    let (input, (_, value, _, bot_id)) =
        tuple((tag("value "), number, tag(" goes to bot "), number))(input)?;
    Ok((input, (bot_id, value)))
}

fn output_id_parser(input: &str) -> IResult<&str, OutputBinId> {
    let (input, output_id) = preceded(tag("output "), number)(input)?;
    Ok((input, output_id))
}

fn value_destination_parser(input: &str) -> IResult<&str, ValueDestination> {
    let (input, value_destination) = alt((
        map(bot_id_parser, |id| ToBot(id)),
        map(output_id_parser, |id| ToOutputBin(id)),
    ))(input)?;
    Ok((input, value_destination))
}

fn bot_id_parser(input: &str) -> IResult<&str, BotId> {
    let (input, bot_id) = preceded(tag("bot "), number)(input)?;
    Ok((input, bot_id))
}

fn bot_rule_parser(input: &str) -> IResult<&str, (BotId, Bot)> {
    let (input, (bot_id, _, low_destination, _, high_destination)) = tuple((
        bot_id_parser,
        tag(" gives low to "),
        value_destination_parser,
        tag(" and high to "),
        value_destination_parser,
    ))(input)?;
    Ok((
        input,
        (
            bot_id,
            Bot {
                value: None,
                gives_low_to: low_destination,
                gives_high_to: high_destination,
            },
        ),
    ))
}

fn entry_parser(input: &str) -> IResult<&str, Entry> {
    let (input, entry) = alt((
        map(bot_rule_parser, |bot| BotRuleEntry(bot)),
        map(initial_value_parser, |initial_value| {
            InitialValueEntry(initial_value)
        }),
    ))(input)?;
    Ok((input, entry))
}

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let (_, entries) = separated_list1(newline, entry_parser)(&file_string).expect("Parse error");
    let (bots, values): (Vec<(BotId, Bot)>, InitialValues) =
        entries.iter().partition_map(|entry| match entry {
            BotRuleEntry(bot_rule) => Either::Left(bot_rule.clone()),
            InitialValueEntry(initial_value) => Either::Right(initial_value),
        });

    let bot_map = bots
        .into_iter()
        .into_grouping_map()
        .aggregate(|_acc, _key, val| Some(val));

    let output_bin_map = bot_map
        .values()
        .flat_map(|bot| vec![bot.gives_high_to.clone(), bot.gives_low_to.clone()])
        .filter_map(|v| match v {
            ToBot(_) => None,
            ToOutputBin(id) => Some((id, OutputBin { values: vec![] })),
        })
        .into_grouping_map()
        .aggregate(|_acc, _key, val| Some(val));

    (bot_map, output_bin_map, values)
}

#[cfg(test)]
mod test {
    use crate::year2016::day10::lib::YEAR_2016_DAY_10_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = YEAR_2016_DAY_10_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = YEAR_2016_DAY_10_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
