use adv_code::year2024::day12::lib::group_gardens_for_all;
use adv_code::year2024::day12::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/2024/12/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 12, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1375574);

    Ok(())
}

fn part1(file_input_path: &str) -> isize {
    let grid = parse_input(&file_input_path);

    let mut garden_map = HashMap::new();
    group_gardens_for_all(&mut garden_map, &grid);

    garden_map
        .values()
        .into_iter()
        .map(|g| g.get_root_garden_struct())
        .unique_by(|g| g.id())
        .map(|garden| garden.area() * garden.perimeter())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::lib::grid_utils::grid_to_str;
    use adv_code::year2024::day12::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/12/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 1930);
    }

    #[test]
    fn test_parser() {
        let grid = parse_input(TEST_INPUT_FILE);
        println!("{}", grid_to_str(&grid));
    }
}
