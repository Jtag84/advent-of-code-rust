use adv_code::lib::grid_utils::set_grid_element;
use adv_code::year2024::day06::lib::get_path;
use adv_code::year2024::day06::lib::parser::parse_input;
use adv_code::year2024::day06::lib::Termination::Loop;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use std::collections::HashSet;

const INPUT_FILE: &str = "input/2024/06/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 06, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1836);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let (guard_position, mut grid) = parse_input(&file_input_path);
    let (path, _) = get_path(guard_position, &grid);

    let mut already_blocked = HashSet::new();
    let mut looped_path_count = 0;
    for (index, (path_coordinates, _)) in (&path).into_iter().enumerate().skip(1) {
        if !already_blocked.insert(path_coordinates) {
            continue;
        }

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
    use adv_code::lib::grid_utils::Direction::{East, North, South};
    use adv_code::year2024::day06::lib::parser::GuardPosition;
    use std::collections::HashSet;

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

    #[test]
    fn test_set() {
        let mut set: HashSet<GuardPosition> = HashSet::new();

        set.insert(((0, 0).into(), North));
        let insert_south = set.insert(((0, 0).into(), South));
        let insert_south_again = set.insert(((0, 0).into(), South));
        let insert_south_different = set.insert(((1, 0).into(), South));

        assert!(insert_south);
        assert!(insert_south_different);
        assert!(!insert_south_again);
        assert!(set.contains(&((0, 0).into(), South)));
        assert!(set.contains(&((1, 0).into(), South)));
        assert!(set.contains(&((0, 0).into(), North)));
        assert!(!set.contains(&((0, 0).into(), East)));
        assert!(!set.contains(&((1, 1).into(), South)));
    }
}
