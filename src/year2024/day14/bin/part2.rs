use adv_code::lib::grid_utils::{grid_to_str, Coordinates, GridCoordinates, XYCoordinates};
use adv_code::year2024::day14::lib::calculate_robots_at_n_seconds;
use adv_code::year2024::day14::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use grid::Grid;
use std::collections::HashSet;

const INPUT_FILE: &str = "input/2024/14/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 14, 2);

    let result = time_snippet!(part2(INPUT_FILE, 101, 103));
    println!("Result = {}", result);

    assert_eq!(result, 6446);

    Ok(())
}

fn part2(file_input_path: &str, map_width: isize, map_height: isize) -> isize {
    let robots = parse_input(&file_input_path);

    for n in 1.. {
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
                    if robots_grid_coordinates.contains(&GridCoordinates::from(grid_coordinates)) {
                        *value = '■';
                    } else {
                        *value = ' '
                    }
                });
            println!("n: {n}");
            println!("{}", grid_to_str(&grid));

            return n;
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod test {
    use adv_code::year2024::day14::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/14/test_inputs_part2.txt";

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
