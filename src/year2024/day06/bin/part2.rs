use adv_code::lib::grid_utils::set_grid_element;
use adv_code::year2024::day06::lib::get_path;
use adv_code::year2024::day06::lib::parser::parse_input;
use adv_code::year2024::day06::lib::Termination::Loop;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/06/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 06, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    // assert_eq!(result, 1);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let (guard_position, mut grid) = parse_input(&file_input_path);
    let (path, _) = get_path(guard_position, &grid);

    let mut looped_path_count = 0;
    for (index, (path_coordinates, _)) in (&path).into_iter().enumerate().skip(1) {
        set_grid_element(&mut grid, path_coordinates, '#');
        let start_guard_position = path.get(index - 1).unwrap();

        let (_, termination) = get_path(*start_guard_position, &grid);
        if termination == Loop {
            looped_path_count += 1;
        }
        set_grid_element(&mut grid, path_coordinates, '.');
    }
    looped_path_count
}

#[cfg(test)]
mod test {
    use crate::parse_input;
    use crate::part2;

    const TEST_INPUT_FILE: &str = "input/2024/06/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 6);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
