use crate::lib::grid_utils::{Coordinates, GridCoordinates};
use grid::Grid;
use std::collections::HashMap;

pub mod parser;

pub type Height = u32;
pub type Position = (GridCoordinates, Height);
pub type PathCount = usize;
pub type NineCount = usize;
pub fn find_hiking_trails(grid: Grid<Height>) -> Vec<(NineCount, PathCount)> {
    let start_position: Vec<Position> = grid
        .indexed_iter()
        .filter(|(_, height)| **height == 0)
        .map(|((row, col), height)| (GridCoordinates::from((row, col)), height.clone()))
        .collect();

    start_position
        .iter()
        .map(|from| {
            let mut visited: HashMap<Position, usize> = HashMap::new();
            let path_count = find_hiking_trails_rec(*from, &grid, &mut visited);
            let nine_count = visited.keys().filter(|(_, height)| *height == 9).count();
            (nine_count, path_count)
        })
        .collect()
}

fn find_hiking_trails_rec(
    from: Position,
    grid: &Grid<Height>,
    visited: &mut HashMap<Position, usize>,
) -> usize {
    let (grid_coordinates, from_height) = from;

    if from_height == 9 {
        return 1;
    }

    vec![
        grid_coordinates.north(),
        grid_coordinates.south(),
        grid_coordinates.east(),
        grid_coordinates.west(),
    ]
    .into_iter()
    .filter(Option::is_some)
    .map(Option::unwrap)
    .map(|coordinates| {
        (
            coordinates,
            grid.get(coordinates.row(), coordinates.column()),
        )
    })
    .filter(|(_, optional_height)| optional_height.is_some())
    .map(|(c, optional_height)| (c, optional_height.unwrap().clone()))
    .filter(|(_, height)| *height == (from_height + 1))
    .map(|new_from| match visited.get(&new_from) {
        Some(count) => count.clone(),
        None => {
            let path_count = find_hiking_trails_rec(new_from, grid, visited);
            visited.insert(new_from, path_count);
            path_count
        }
    })
    .filter(|v| *v > 0)
    .sum()
}
