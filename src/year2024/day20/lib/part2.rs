use crate::lib::grid_utils::Coordinates;
use crate::year2024::day20::lib::parser::ParsedInput;
use pathfinding::prelude::astar;

pub fn part2(parsed_input: ParsedInput, cheat_cost_saving: usize) -> String {
    calculate_count_of_cheats_saving(parsed_input, cheat_cost_saving, 20).to_string()
}

// This method could also be used for part1, but the current part1 implementation is much faster
fn calculate_count_of_cheats_saving(
    (racetrack, start, end): ParsedInput,
    cheat_cost_saving: usize,
    cheat_max_length: usize,
) -> usize {
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
    use crate::year2024::day20::lib::part2::part2;
    use crate::year2024::day20::lib::YEAR_2024_DAY_20_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_20_SOLUTION.get_parsed_test_inputs(2), 70),
            "41"
        );
    }
}
