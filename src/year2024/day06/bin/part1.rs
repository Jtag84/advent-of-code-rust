use adv_code::year2024::day06;
use adv_code::year2024::day06::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use day06::lib::get_path;
use std::collections::HashSet;

const INPUT_FILE: &str = "input/2024/06/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 06, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 5461);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let (guard_position, grid) = parse_input(&file_input_path);
    let (path, _) = get_path(guard_position, &grid);
    path.iter()
        .map(|(guard_coordinates, _)| guard_coordinates)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day06::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/06/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 41);
    }

    #[test]
    fn test_parser() {
        let (guard_position, grid) = parse_input(TEST_INPUT_FILE);
        println!("guard_position = {:?}", guard_position);
        println!("{}", grid_to_str(&grid));
    }
}
