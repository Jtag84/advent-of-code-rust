use crate::lib::grid_utils::{GridCoordinates, Position};
use grid::Grid;

pub mod parser;

pub type Cost = usize;

pub fn successors(
    grid: &Grid<char>,
    position: &Position<GridCoordinates>,
) -> Vec<(Position<GridCoordinates>, Cost)> {
    vec![
        position
            .move1()
            .filter(|p| {
                grid.get(p.coordinates().row(), p.coordinates().column())
                    .filter(|v| **v == '.')
                    .is_some()
            })
            .map(|p| (p, 1)),
        Some((position.rotate_clockwise_90(), 1000)),
        Some((position.rotate_counter_clockwise_90(), 1000)),
    ]
    .iter()
    .filter_map(|p| *p)
    .collect()
}
