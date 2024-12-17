use crate::lib::parser_commons::read_file_to_string;
use nom::character::complete::digit1;
use nom::IResult;

fn str_number_parser(input: &str) -> IResult<&str, &str> {
    let (input, digits) = digit1(input)?;

    Ok((input, digits))
}

pub fn parse_input(input_path: &str) -> Vec<usize> {
    let file_string = read_file_to_string(input_path);

    let digits_result = str_number_parser(&file_string);
    digits_result
        .expect("Parsing error")
        .1
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}
