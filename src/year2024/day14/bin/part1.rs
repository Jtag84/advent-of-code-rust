use adv_code::lib::grid_utils::XYCoordinates;
use adv_code::year2024::day14::lib::calculate_robots_at_n_seconds;
use adv_code::year2024::day14::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/14/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 14, 1);

    let result = time_snippet!(part1(INPUT_FILE, 101, 103));
    println!("Result = {}", result);

    assert_eq!(result, 218619324);

    Ok(())
}

fn part1(file_input_path: &str, map_width: isize, map_height: isize) -> usize {
    let robots = parse_input(&file_input_path);
    let width_middle = map_width / 2;
    let height_middle = map_height / 2;
    let robots_after_100s = calculate_robots_at_n_seconds(map_width, map_height, &robots, 100);

    let (left_half, right_half): (Vec<XYCoordinates>, Vec<XYCoordinates>) = robots_after_100s
        .iter()
        .filter(|XYCoordinates(x, y)| *x != width_middle && *y != height_middle)
        .partition(|XYCoordinates(x, _)| *x < width_middle);

    let quadrants = [left_half, right_half]
        .into_iter()
        .map(|half| {
            half.iter()
                .partition(|XYCoordinates(_, y)| *y < height_middle)
        })
        .collect::<Vec<_>>();

    quadrants
        .into_iter()
        .map(
            |(up_half, down_half): (Vec<XYCoordinates>, Vec<XYCoordinates>)| {
                up_half.len() * down_half.len()
            },
        )
        .reduce(|accum, item| accum * item)
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day14::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/14/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE, 11, 7), 12);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
