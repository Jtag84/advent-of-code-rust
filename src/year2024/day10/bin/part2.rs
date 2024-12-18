use adv_code::year2024::day10::lib::find_hiking_trails;
use adv_code::year2024::day10::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/10/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 10, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {:?}", result);

    assert_eq!(result, 1094);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let grid = parse_input(&file_input_path);

    find_hiking_trails(grid)
        .iter()
        .map(|(_, path_count)| path_count)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day10::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/10/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        let score = part2(TEST_INPUT_FILE);
        assert_eq!(score, 81);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
