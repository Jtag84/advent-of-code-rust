use crate::lib::parser_commons::{number, read_file_to_string};

pub type ParsedInput = usize;

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);

    let (_, result) = number(&file_string).expect("Parse error");
    result
}

#[cfg(test)]
mod test {
    use crate::year2016::day13::lib::YEAR_2016_DAY_13_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = YEAR_2016_DAY_13_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = YEAR_2016_DAY_13_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
