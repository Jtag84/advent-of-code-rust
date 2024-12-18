use adv_code::year2024::day10::lib::find_hiking_trails;
use adv_code::year2024::day10::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/10/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 10, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 482);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let grid = parse_input(&file_input_path);

    find_hiking_trails(grid)
        .iter()
        .map(|(nine_count, _)| nine_count)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day10::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/10/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 36);
    }

    #[test]
    fn test_parser() {
        let grid = parse_input(TEST_INPUT_FILE);
        println!("{}", grid_to_str(&grid));
    }
}
