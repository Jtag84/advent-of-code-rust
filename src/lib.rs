use nom::Parser;
use std::fs::File;
use std::io::Read;

pub mod year2024;

pub fn start_day(year: i32, day: i32, part: i32) {
    println!("Advent of Code {year} - Day {:0>2}", day);
    println!("=== Part {part} ===");
}

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day(2024, 0, 0);
    }
}
