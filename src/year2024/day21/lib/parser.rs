use crate::lib::parser_commons::read_file_to_string;

pub type ParsedInput = Vec<String>;
pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);
    file_string
        .lines()
        .into_iter()
        .map(|line| line.to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::year2024::day21::lib::YEAR_2024_DAY_21_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let parsed = YEAR_2024_DAY_21_SOLUTION.get_parsed_test_inputs(1);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_parser_part2() {
        let parsed = YEAR_2024_DAY_21_SOLUTION.get_parsed_test_inputs(2);
        println!("Result = {:?}", parsed);
    }
}
