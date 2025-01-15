use crate::year2024::day18::lib::find_shortest_path_after_dropping_n_bytes;
use crate::year2024::day18::lib::parser::ParsedInput;

pub fn part1(bytes: ParsedInput, grid_size: usize, num_bytes_to_drop: usize) -> usize {
    let path = find_shortest_path_after_dropping_n_bytes(&bytes, grid_size, num_bytes_to_drop);
    path.unwrap().1
}

#[cfg(test)]
mod test {
    use crate::year2024::day18::lib::part1::part1;
    use crate::year2024::day18::lib::YEAR_2024_DAY_18_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_18_SOLUTION.get_parsed_test_inputs(1), 7, 12),
            22
        );
    }
}
