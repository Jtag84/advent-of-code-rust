use crate::lib::grid_utils::{Coordinates, Direction, Position};
use crate::year2024::day16::lib::parser::ParsedInput;
use crate::year2024::day16::lib::successors;
use pathfinding::prelude::astar;

pub fn part1((grid, start, end): ParsedInput) -> String {
    let start_position = Position(start, Direction::East);
    let shortest_path = astar(
        &start_position,
        |p| successors(&grid, p),
        |p| p.coordinates().manhattan_distance(end),
        |p| p.coordinates() == end,
    )
    .unwrap();
    shortest_path.1.to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day16::lib::part1::part1;
    use crate::year2024::day16::lib::YEAR_2024_DAY_16_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_16_SOLUTION.get_parsed_test_inputs(1)),
            "11048"
        );
    }
}
