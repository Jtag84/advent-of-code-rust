use crate::lib::grid_utils::{Coordinates, XYCoordinates, X, Y};
use crate::year2016::day13::lib::parser::ParsedInput;
use itertools::Itertools;
use pathfinding::prelude::astar;

pub fn part1(designers_number: ParsedInput, goal: XYCoordinates) -> String {
    astar(
        &XYCoordinates(1, 1),
        |node| {
            node.cardinals()
                .iter()
                .filter(|XYCoordinates(x, y)| is_empty_space(designers_number, x, y))
                .map(|next| (*next, 1))
                .collect_vec()
        },
        |node| node.manhattan_distance(goal),
        |node| *node == goal,
    )
    .unwrap()
    .1
    .to_string()
}

pub fn is_empty_space(designers_number: usize, x: &X, y: &Y) -> bool {
    let calculation = x * x + 3 * x + 2 * x * y + y + y * y + designers_number as isize;
    let number_of_ones = format!("{calculation:b}")
        .chars()
        .filter(|c| *c == '1')
        .count();
    *x >= 0 && *y >= 0 && number_of_ones % 2 == 0
}

#[cfg(test)]
mod test {
    use crate::lib::grid_utils::XYCoordinates;
    use crate::year2016::day13::lib::part1::part1;
    use crate::year2016::day13::lib::YEAR_2016_DAY_13_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                YEAR_2016_DAY_13_SOLUTION.get_parsed_test_inputs(1),
                XYCoordinates(7, 4)
            ),
            "11"
        );
    }
}
