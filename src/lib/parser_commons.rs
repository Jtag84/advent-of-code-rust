use nom::{IResult, Parser};
use std::fs::File;
use std::io::Read;
use nom::character::complete::digit1;
use nom::combinator::map_res;

pub fn read_file_to_string(input_path: &str) -> String {
    let mut file_string = String::new();
    File::open(input_path).expect("Can't open file")
        .read_to_string(&mut file_string).expect("can't read file");
    file_string
}

// Keeps applying the parser until the end of the input, skipping a character when the parser doesn't match
pub fn find_all<'a, O, E, P>(input: &'a str, mut parser: P ) -> Vec<O>
where
    P: Parser<&'a str, O, E>
{
    let mut results = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        match parser.parse(remaining) {
            Ok((rest, matched)) => {
                results.push(matched);
                remaining = rest;
            }
            Err(_) => {
                // Skip one character if no match is found
                remaining = &remaining[1..];
            }
        }
    }
    results
}

pub fn number<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse)(input)
}