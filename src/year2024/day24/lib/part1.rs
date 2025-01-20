use crate::year2024::day24::lib::parser::{ParsedInput, WireName, WireState};
use itertools::Itertools;
use std::ops::{BitAnd, BitOr, BitXor};

pub fn part1(wire_state_map: ParsedInput) -> String {
    let zs = wire_state_map
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .rev()
        .collect_vec();

    let binary_output = zs
        .into_iter()
        .map(|z| get_bit_value(&wire_state_map, z))
        .map(|bit| if bit { "1" } else { "0" })
        .join("");

    usize::from_str_radix(binary_output.as_str(), 2)
        .unwrap()
        .to_string()
}

fn get_bit_value(wire_state_map: &ParsedInput, wire_name: &WireName) -> bool {
    let wire_state = wire_state_map.get(wire_name).unwrap();

    match wire_state {
        WireState::AND(left_wire, right_wire) => get_bit_value(wire_state_map, left_wire)
            .bitand(get_bit_value(wire_state_map, right_wire)),
        WireState::OR(left_wire, right_wire) => get_bit_value(wire_state_map, left_wire)
            .bitor(get_bit_value(wire_state_map, right_wire)),
        WireState::XOR(left_wire, right_wire) => get_bit_value(wire_state_map, left_wire)
            .bitxor(get_bit_value(wire_state_map, right_wire)),
        WireState::ONE => true,
        WireState::ZERO => false,
    }
}

#[cfg(test)]
mod test {
    use crate::year2024::day24::lib::part1::part1;
    use crate::year2024::day24::lib::YEAR_2024_DAY_24_SOLUTION;

    #[test]
    fn part1_input() {
        assert_eq!(part1(YEAR_2024_DAY_24_SOLUTION.get_parsed_inputs()), "2024");
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_24_SOLUTION.get_parsed_test_inputs(1)),
            "2024"
        );
    }
}
