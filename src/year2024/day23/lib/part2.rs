use crate::year2024::day23::lib::parser::{Computer, ParsedInput};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::once;

pub fn part2(connections_map: ParsedInput) -> String {
    connections_map
        .clone()
        .into_iter()
        .flat_map(|(computer, connections)| {
            let combination_size = connections.len();
            connections
                .into_iter()
                .chain(once(computer.clone()))
                .combinations(combination_size)
                .map(|connection_list| connection_list.clone().into_iter().collect::<HashSet<_>>())
        })
        .unique_by(|connection| connection.iter().sorted().join(""))
        .filter_map(|connections| {
            let all_connected = connections
                .iter()
                .all(|c| is_connected_to_all(connections_map.clone(), &connections.clone(), c));
            if all_connected {
                Some(connections.clone())
            } else {
                None
            }
        })
        .max_by(|set1, set2| set1.len().cmp(&set2.len()))
        .unwrap()
        .into_iter()
        .sorted()
        .join(",")
}

fn is_connected_to_all(
    connections_map: ParsedInput,
    connections: &HashSet<Computer>,
    computer: &Computer,
) -> bool {
    if let Some(computer_connections) = connections_map.get(computer) {
        let connections_with_computer = &computer_connections
            .iter()
            .chain(once(computer))
            .cloned()
            .collect();
        connections.is_subset(connections_with_computer)
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::year2024::day23::lib::part2::part2;
    use crate::year2024::day23::lib::YEAR_2024_DAY_23_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_23_SOLUTION.get_parsed_test_inputs(1)),
            "co,de,ka,ta"
        );
    }

    #[test]
    fn part2_full_inputs_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_23_SOLUTION.get_parsed_inputs()),
            "cb,df,fo,ho,kk,nw,ox,pq,rt,sf,tq,wi,xz"
        );
    }
}
