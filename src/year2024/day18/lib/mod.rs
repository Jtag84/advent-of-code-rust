use crate::lib::grid_utils::{Coordinates, XYCoordinates};
use pathfinding::prelude::astar;
use std::collections::HashSet;

pub mod parser;

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
