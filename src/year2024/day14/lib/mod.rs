use crate::lib::grid_utils::XYCoordinates;
use crate::year2024::day14::lib::parser::PositionAndSpeed;

pub mod parser;

pub fn calculate_robots_at_n_seconds(
    map_width: isize,
    map_height: isize,
    robots: &Vec<PositionAndSpeed>,
    n: isize,
) -> Vec<XYCoordinates> {
    robots
        .iter()
        .map(|robot| calculate_position_at_n_seconds(robot, map_width, map_height, n))
        .collect::<Vec<XYCoordinates>>()
}

pub fn calculate_position_at_n_seconds(
    (XYCoordinates(x, y), XYCoordinates(vx, vy)): &PositionAndSpeed,
    map_width: isize,
    map_height: isize,
    n: isize,
) -> XYCoordinates {
    XYCoordinates(
        (x + vx * n).rem_euclid(map_width),
        (y + vy * n).rem_euclid(map_height),
    )
}
