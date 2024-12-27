use adv_code::year2024::day12::lib::group_gardens_for_all;
use adv_code::year2024::day12::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/2024/12/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 12, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 830566);

    Ok(())
}

fn part2(file_input_path: &str) -> isize {
    let grid = parse_input(&file_input_path);

    let mut garden_map = HashMap::new();
    group_gardens_for_all(&mut garden_map, &grid);

    garden_map
        .values()
        .into_iter()
        .map(|g| g.get_root_garden_struct())
        .unique_by(|g| g.id())
        .map(|garden| {
            let corner_count: isize = garden.corner_count().try_into().unwrap();
            garden.area() * corner_count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day12::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/12/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 368);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
