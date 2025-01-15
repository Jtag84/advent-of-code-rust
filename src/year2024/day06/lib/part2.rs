use crate::lib::grid_utils::set_grid_element;
use crate::year2024::day06::lib::get_path;
use crate::year2024::day06::lib::parser::ParsedInput;
use crate::year2024::day06::lib::Termination::Loop;
use std::collections::HashSet;

pub fn part2((guard_position, mut grid): ParsedInput) -> String {
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
    looped_path_count.to_string()
}

#[cfg(test)]
mod test {
    use crate::lib::grid_utils::Direction::{East, North, South};
    use crate::year2024::day06::lib::parser::GuardPosition;
    use crate::year2024::day06::lib::{part2, YEAR_2024_DAY_06_SOLUTION};
    use std::collections::HashSet;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_06_SOLUTION.get_parsed_test_inputs(2)),
            "6"
        );
    }

    #[test]
    fn test_set() {
        let mut set: HashSet<GuardPosition> = HashSet::new();

        set.insert(((0_isize, 0).into(), North));
        let insert_south = set.insert(((0_isize, 0).into(), South));
        let insert_south_again = set.insert(((0_isize, 0).into(), South));
        let insert_south_different = set.insert(((1_isize, 0).into(), South));

        assert!(insert_south);
        assert!(insert_south_different);
        assert!(!insert_south_again);
        assert!(set.contains(&((0_isize, 0).into(), South)));
        assert!(set.contains(&((1_isize, 0).into(), South)));
        assert!(set.contains(&((0_isize, 0).into(), North)));
        assert!(!set.contains(&((0_isize, 0).into(), East)));
        assert!(!set.contains(&((1_isize, 1).into(), South)));
    }
}
