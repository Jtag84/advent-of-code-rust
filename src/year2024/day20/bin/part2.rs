use adv_code::lib::grid_utils::Coordinates;
use adv_code::year2024::day20::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use pathfinding::prelude::astar;

const INPUT_FILE: &str = "input/2024/20/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 20, 2);

    let result = time_snippet!(part2(INPUT_FILE, 100));
    println!("Result = {}", result);

    assert_eq!(result, 982891);

    Ok(())
}

fn part2(file_input_path: &str, cheat_cost_saving: usize) -> usize {
    calculate_count_of_cheats_saving(&file_input_path, cheat_cost_saving, 20)
}

// This method could also be used for part1, but the current part1 implementation is much faster
fn calculate_count_of_cheats_saving(
    file_input_path: &&str,
    cheat_cost_saving: usize,
    cheat_max_length: usize,
) -> usize {
    let (racetrack, start, end) = parse_input(&file_input_path);
    let (no_cheat_path, _) = astar(
        &start,
        |node| {
            node.cardinals().into_iter().filter_map(|c| {
                racetrack
                    .get(c.row(), c.column())
                    .filter(|v| **v == '.')
                    .map(|_| (c, 1))
            })
        },
        |node| node.manhattan_distance(end),
        |node| *node == end,
    )
    .unwrap();

    let mut connecting_path_with_cheat_count = 0;
    for (from_index, from_coordinates) in no_cheat_path.clone().into_iter().enumerate() {
        for (to_index, to_coordinates) in no_cheat_path
            .clone()
            .into_iter()
            .enumerate()
            .skip(from_index + cheat_cost_saving)
        {
            let manhattan_distance = from_coordinates.manhattan_distance(to_coordinates);
            let cost_saving = to_index - from_index - manhattan_distance;
            if manhattan_distance <= cheat_max_length && cost_saving >= cheat_cost_saving {
                connecting_path_with_cheat_count += 1
            }
        }
    }
    connecting_path_with_cheat_count
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day20::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/20/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE, 70), 41);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
