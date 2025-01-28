use crate::lib::grid_utils::{Coordinates, XYCoordinates};
use crate::year2024::day18::lib::parser::{parse_input, ParsedInput};
pub use crate::year2024::day18::lib::part1::part1;
pub use crate::year2024::day18::lib::part2::part2;
use crate::SOLUTIONS;
use crate::{aoc_solver, DaySolution};
use pathfinding::prelude::astar;
use std::collections::HashSet;

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_18_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 18),
    parser: parse_input,
    part1: |parsed_input| part1(parsed_input, 71, 1024).to_string(),
    expected_part1: "404",
    part2: |parsed_input| part2(parsed_input, 71, 1024).to_string(),
    expected_part2: "XYCoordinates(27,60)",
};

aoc_solver!(YEAR_2024_DAY_18_SOLUTION);

pub fn find_shortest_path_after_dropping_n_bytes(
    bytes_list: &Vec<XYCoordinates>,
    grid_size: usize,
    num_bytes_to_drop: usize,
) -> Option<(Vec<XYCoordinates>, usize)> {
    let bytes: HashSet<&XYCoordinates> = bytes_list.into_iter().take(num_bytes_to_drop).collect();

    let start = XYCoordinates(0, 0);
    let max_boundary = grid_size as isize - 1;
    let goal = XYCoordinates(max_boundary, max_boundary);
    let path = astar(
        &start,
        |xy| {
            xy.cardinals()
                .into_iter()
                .filter(|XYCoordinates(x, y)| {
                    *x >= 0 && *x < grid_size as isize && *y >= 0 && *y < grid_size as isize
                })
                .filter(|c| !bytes.contains(c))
                .map(|c| (c, 1))
        },
        |xy| xy.manhattan_distance(goal),
        |xy| *xy == goal,
    );
    path
}
