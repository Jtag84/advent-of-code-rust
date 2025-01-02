use adv_code::year2024::day18::lib::find_shortest_path_after_dropping_n_bytes;
use adv_code::year2024::day18::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/18/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 18, 1);

    let result = time_snippet!(part1(INPUT_FILE, 71, 1024));
    println!("Result = {}", result);

    assert_eq!(result, 404);

    Ok(())
}

fn part1(file_input_path: &str, grid_size: usize, num_bytes_to_drop: usize) -> usize {
    let bytes = parse_input(&file_input_path);
    let path = find_shortest_path_after_dropping_n_bytes(&bytes, grid_size, num_bytes_to_drop);
    path.unwrap().1
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day18::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/18/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE, 7, 12), 22);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
