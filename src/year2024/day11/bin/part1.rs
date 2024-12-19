use adv_code::year2024::day11::lib::blink_n;
use adv_code::year2024::day11::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/11/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 11, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 190865);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let stones = parse_input(&file_input_path);
    blink_n(stones, 25)
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day11::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/11/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 55312);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_scan() {
        let offsets = vec![3, 2, 1, 4];
        let positions: Vec<i32> = offsets
            .iter()
            .scan(0, |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect();
        println!("{:?}", positions);
    }
}
