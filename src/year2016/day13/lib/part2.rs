use crate::lib::grid_utils::{Coordinates, XYCoordinates};
use crate::year2016::day13::lib::parser::ParsedInput;
use crate::year2016::day13::lib::part1::is_empty_space;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part2(designers_number: ParsedInput) -> String {
    let start = XYCoordinates(1, 1);
    let mut visited = HashSet::new();
    visited.insert(start);
    find_reachable_locations(&start, &mut visited, 50, designers_number);
    visited.len().to_string()
}

fn find_reachable_locations(
    from_coordinate: &XYCoordinates,
    visited: &mut HashSet<XYCoordinates>,
    steps_left: usize,
    designers_number: usize,
) {
    if steps_left == 0 {
        return;
    }

    let new_reachable_locations = from_coordinate
        .cardinals()
        .into_iter()
        .filter(|c| !visited.contains(c))
        .filter(|XYCoordinates(x, y)| is_empty_space(designers_number, x, y))
        .collect_vec();

    new_reachable_locations
        .iter()
        .cloned()
        .for_each(|new_location| {
            visited.insert(new_location);
        });

    for new_location in new_reachable_locations {
        find_reachable_locations(&new_location, visited, steps_left - 1, designers_number);
    }
}

#[cfg(test)]
mod test {
    use crate::year2016::day13::lib::part2::part2;
    use crate::year2016::day13::lib::YEAR_2016_DAY_13_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2016_DAY_13_SOLUTION.get_parsed_test_inputs(2)),
            "148"
        );
    }
}
