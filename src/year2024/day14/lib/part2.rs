use crate::lib::grid_utils::{Coordinates, GridCoordinates, XYCoordinates};
use crate::year2024::day14::lib::calculate_robots_at_n_seconds;
use crate::year2024::day14::lib::parser::ParsedInput;
use grid::Grid;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::collections::HashSet;

pub fn part2(robots: ParsedInput, map_width: isize, map_height: isize) -> String {
    let res = (1..10000)
        .into_par_iter()
        .filter_map(|n| {
            let robots_after_n_seconds_list =
                &calculate_robots_at_n_seconds(map_width, map_height, &robots, n);
            let robots_after_n_seconds_set: &HashSet<&XYCoordinates> =
                &(robots_after_n_seconds_list.iter().collect());

            let robots_with_a_neighbor = robots_after_n_seconds_list
                .into_iter()
                .flat_map(|c| c.cardinals())
                .filter(|coords| robots_after_n_seconds_set.contains(coords))
                .count();

            let robot_cluster_threshold = robots_after_n_seconds_list.len() / 2 * 3;
            if robots_with_a_neighbor >= robot_cluster_threshold {
                let mut grid: Grid<char> = Grid::new(map_height as usize, map_width as usize);
                let robots_grid_coordinates: HashSet<GridCoordinates> = robots_after_n_seconds_list
                    .into_iter()
                    .map(|xy_coordinates| GridCoordinates::from(*xy_coordinates))
                    .collect();

                grid.indexed_iter_mut()
                    .for_each(|(grid_coordinates, value)| {
                        if robots_grid_coordinates
                            .contains(&GridCoordinates::from(grid_coordinates))
                        {
                            *value = '■';
                        } else {
                            *value = ' '
                        }
                    });
                // println!("n: {n}");
                // println!("{}", grid_to_str(&grid));

                Some(n.to_string())
            } else {
                None
            }
        })
        .take_any(1)
        .collect::<Vec<String>>();

    if res.len() == 1 {
        return res[0].to_string();
    }

    panic!("No solution found");
}
