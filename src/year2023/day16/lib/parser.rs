use crate::lib::parser_commons::parse_grid;
use grid::Grid;

pub type ParsedInput = Grid<char>;

pub fn parse_input(input_path: &str) -> Grid<char> {
    parse_grid(input_path)
}

#[cfg(test)]
mod test {
    use crate::lib::grid_utils::grid_to_str;
    use crate::year2023::day16::lib::YEAR_2023_DAY_16_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let grid = YEAR_2023_DAY_16_SOLUTION.get_parsed_test_inputs(1);
        println!("{}", grid_to_str(&grid));
    }

    #[test]
    fn test_parser_part2() {
        let grid = YEAR_2023_DAY_16_SOLUTION.get_parsed_test_inputs(2);
        println!("{}", grid_to_str(&grid));
    }
}
