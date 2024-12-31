use grid::Grid;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt, recognize};
use nom::sequence::preceded;
use nom::{IResult, Parser};
use std::fs::File;
use std::io::Read;

pub fn read_file_to_string(input_path: &str) -> String {
    let mut file_string = String::new();
    File::open(input_path)
        .expect("Can't open file")
        .read_to_string(&mut file_string)
        .expect("can't read file");
    file_string
}

pub fn parse_grid_digit<T: Clone + From<u32>>(input_path: &str) -> Grid<T> {
    parse_grid_with(input_path, |c| c.to_digit(10).unwrap().try_into().unwrap())
}

pub fn parse_grid<T>(input_path: &str) -> Grid<T>
where
    T: From<char> + Clone,
{
    parse_grid_with(input_path, |c| c.try_into().unwrap())
}

pub fn parse_grid_with<T, P>(input_path: &str, parser: P) -> Grid<T>
where
    T: Clone,
    P: Fn(char) -> T,
{
    let file_string = read_file_to_string(input_path);
    parse_grid_from_string_with(file_string, parser)
}

pub fn parse_grid_from_string<T>(file_string: String) -> Grid<T>
where
    T: From<char> + Clone,
{
    parse_grid_from_string_with(file_string, |c| c.try_into().unwrap())
}

pub fn parse_grid_from_string_with<T, P>(file_string: String, parser: P) -> Grid<T>
where
    T: Clone,
    P: Fn(char) -> T,
{
    let parsed_input: Vec<Vec<T>> = file_string
        .lines()
        .map(|l| l.chars().map(|c| parser(c)).collect())
        .collect();
    Grid::from(&parsed_input)
}

// Keeps applying the parser until the end of the input, skipping a character when the parser doesn't match
pub fn find_all<'a, O, E, P>(input: &'a str, mut parser: P) -> Vec<O>
where
    P: Parser<&'a str, O, E>,
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
    map_res(
        recognize(preceded(opt(alt((char('-'), char('+')))), digit1)),
        |s: &str| s.parse(),
    )(input)
}
