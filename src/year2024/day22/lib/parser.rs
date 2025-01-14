use crate::lib::parser_commons::{number, read_file_to_string};
use nom::character::complete::newline;
use nom::multi::separated_list1;

pub fn parse_input(input_path: &str) -> Vec<usize> {
    let file_string = read_file_to_string(input_path);

    let parsed_columns_result = separated_list1(newline, number)(&file_string);
    parsed_columns_result.expect("Parsing error").1
}

#[cfg(test)]
mod test {
    use crate::year2024::day22::lib::DAY_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = DAY_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = DAY_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
