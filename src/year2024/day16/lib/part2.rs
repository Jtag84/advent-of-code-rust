use crate::lib::grid_utils::{Coordinates, Direction, Position};
use crate::year2024::day16::lib::parser::ParsedInput;
use crate::year2024::day16::lib::successors;
use itertools::Itertools;
use pathfinding::prelude::astar_bag_collect;

pub fn part2((grid, start, end): ParsedInput) -> String {
    let start_position = Position(start, Direction::East);
    let shortest_paths = astar_bag_collect(
        &start_position,
        |p| successors(&grid, p),
        |p| p.coordinates().manhattan_distance(end),
        |p| p.coordinates() == end,
    )
    .unwrap();
    shortest_paths
        .0
        .into_iter()
        .flatten()
        .map(|p| p.coordinates())
        .unique()
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day16::lib::part2::part2;
    use crate::year2024::day16::lib::YEAR_2024_DAY_16_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_16_SOLUTION.get_parsed_test_inputs(2)),
            "64"
        );
    }
}
