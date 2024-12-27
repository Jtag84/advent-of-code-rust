use crate::lib::parser_commons::{number, read_file_to_string};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, newline};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

pub type X = isize;
pub type Y = isize;
pub type XYCoordinates = (X, Y);
pub type ButtonA = XYCoordinates;
pub type ButtonB = XYCoordinates;
pub type Prize = XYCoordinates;

pub type MachineBehavior = (ButtonA, ButtonB, Prize);

fn button_parser(button_name: char) -> impl Fn(&str) -> IResult<&str, XYCoordinates> {
    move |input| {
        let tag_prefix = format!("Button {button_name}: X+");
        let (input, (_, x, _, y)) =
            tuple((tag(tag_prefix.as_str()), number, tag(", Y+"), number))(input)?;
        Ok((input, (x, y)))
    }
}

fn prize_parser(input: &str) -> IResult<&str, Prize> {
    let (input, (_, x, _, y)) = tuple((tag("Prize: X="), number, tag(", Y="), number))(input)?;
    Ok((input, (x, y)))
}

fn machine_behavior_parser(input: &str) -> IResult<&str, MachineBehavior> {
    let (input, (button_a, _, button_b, _, prize)) = tuple((
        button_parser('A'),
        newline,
        button_parser('B'),
        newline,
        prize_parser,
    ))(input)?;
    Ok((input, (button_a, button_b, prize)))
}

pub fn parse_input(input_path: &str) -> Vec<MachineBehavior> {
    let file_string = read_file_to_string(input_path);

    let result = separated_list1(multispace1, machine_behavior_parser)(&file_string);
    result.expect("Parsing error").1
}
