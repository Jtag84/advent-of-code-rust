use crate::year2024::day23::lib::parser::{Computer, ParsedInput};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::once;

pub fn part1(connections_map: ParsedInput) -> String {
    let visited = &mut HashSet::new();
    connections_map
        .keys()
        .map(|c| {
            count_number_of_3_computers_with_one_name_starting_with_a_t(
                &connections_map,
                HashSet::new(),
                c.clone(),
                visited,
            )
        })
        .sum::<usize>()
        .to_string()
}

fn count_number_of_3_computers_with_one_name_starting_with_a_t(
    connections_map: &ParsedInput,
    current_set: HashSet<Computer>,
    current_computer: Computer,
    already_visited: &mut HashSet<String>,
) -> usize {
    let new_set: HashSet<Computer> = current_set
        .iter()
        .chain(once(&current_computer))
        .cloned()
        .collect();

    let already_visited_key = new_set.iter().sorted().join("");
    if already_visited.contains(&already_visited_key) {
        return 0;
    }

    if current_set.len() == 2 {
        already_visited.insert(already_visited_key);
        return if (current_set.iter().any(|c| c.starts_with("t"))
            || current_computer.starts_with("t"))
            && connections_map
                .get(&current_computer)
                .filter(|computers| current_set.intersection(computers).count() == 2)
                .is_some()
        {
            1
        } else {
            0
        };
    }

    match connections_map.get(&current_computer) {
        Some(connections) => connections
            .iter()
            .map(|c| {
                count_number_of_3_computers_with_one_name_starting_with_a_t(
                    connections_map,
                    new_set.clone(),
                    c.clone(),
                    already_visited,
                )
            })
            .sum(),
        None => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::year2024::day23::lib::part1::part1;
    use crate::year2024::day23::lib::YEAR_2024_DAY_23_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_23_SOLUTION.get_parsed_test_inputs(1)),
            "7"
        );
    }
}
