use crate::lib::grid_utils::{Coordinates, Direction, GridCoordinates};
use crate::year2024::day06::lib::parser::GuardPosition;
use grid::Grid;
use std::collections::{HashSet, VecDeque};

pub mod parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Termination {
    ExitedMap,
    Loop,
}

pub fn get_path(
    (start_guard_coordinates, start_direction): GuardPosition,
    grid: &Grid<char>,
) -> (VecDeque<GuardPosition>, Termination) {
    let mut path = VecDeque::new();
    path.push_back((start_guard_coordinates, start_direction));
    let mut visited = HashSet::new();

    let mut guard_coordinates = start_guard_coordinates;
    let mut direction = start_direction;
    loop {
        let new_coordinates_option = guard_coordinates.move_to(direction);
        let new_coordinates = match new_coordinates_option {
            Some(coordinates) => {
                coordinates
            }
            None => {
                let guard_position = (guard_coordinates, direction);
                path.push_back(guard_position);
                return (path, Termination::ExitedMap);
            }
        };

        let map_element = grid.get(new_coordinates.row(), new_coordinates.column());
        match map_element {
            Some('.') => {
                let guard_position = (new_coordinates, direction);
                if !visited.insert(guard_position.clone()) {
                    return (path, Termination::Loop);
                }
                path.push_back(guard_position);
                guard_coordinates = new_coordinates;
            }
            Some('#') => {
                direction = direction.rotate_clockwise_90();
            }
            Some(_) => panic!("Unexpected element in map"),
            None => {
                path.push_back((guard_coordinates, direction));
                return (path, Termination::ExitedMap);
            }
        }
    }
}
