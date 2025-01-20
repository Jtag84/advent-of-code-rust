use crate::year2024::day24::lib::parser::WireState::{ONE, ZERO};
use crate::year2024::day24::lib::parser::{ParsedInput, WireName, WireState};
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::{BitAnd, BitOr, BitXor};
use WireState::{AND, OR, XOR};

pub fn part2<F>(
    wire_state_map: ParsedInput,
    expected_operation: F,
    xy_inputs: Vec<(usize, usize)>,
) -> String
where
    F: Fn(usize, usize) -> usize,
{
    let all_outputs_to_swap =
        get_wrong_bits_for_inputs(&wire_state_map, &expected_operation, &xy_inputs);

    let generic1 = all_outputs_to_swap
        .clone()
        .into_iter()
        .combinations(2)
        .collect_vec();

    let generic = generic1
        .into_iter()
        // .skip(1)
        // .take(1)
        .filter(|swap| {
            let mut swapped_wire_state_map = wire_state_map.clone();
            let s0 = wire_state_map.get(&swap[0]).unwrap();
            let s1 = wire_state_map.get(&swap[1]).unwrap();
            swapped_wire_state_map.insert(swap[0].clone(), s1.clone());
            swapped_wire_state_map.insert(swap[1].clone(), s0.clone());

            get_wrong_bits_for_inputs(&swapped_wire_state_map, &expected_operation, &xy_inputs)
                .len()
                < all_outputs_to_swap.len()
        })
        .collect_vec();

    let all_possible_combinations = generic
        .into_iter()
        .combinations(4)
        .filter(|combination| {
            combination.into_iter().flatten().unique().count() == combination.len() * 2
        })
        .collect_vec();

    let valid_swaps = all_possible_combinations
        .iter()
        .filter(|swaps| {
            let mut swapped_wire_state_map = wire_state_map.clone();
            swaps.into_iter().for_each(|swap| {
                let s0 = wire_state_map.get(&swap[0]).unwrap();
                let s1 = wire_state_map.get(&swap[1]).unwrap();
                swapped_wire_state_map.insert(swap[0].clone(), s1.clone());
                swapped_wire_state_map.insert(swap[1].clone(), s0.clone());
            });

            get_wrong_bits_for_inputs(&swapped_wire_state_map, &expected_operation, &xy_inputs)
                .is_empty()
        })
        .collect_vec();
    // .exactly_one()
    // .unwrap();

    valid_swaps[0]
        .iter()
        .flatten()
        .sorted()
        .join(",")
        .to_string()
}

fn get_wrong_bits_for_inputs<F>(
    wire_state_map: &ParsedInput,
    expected_operation: &F,
    xy_inputs: &Vec<(usize, usize)>,
) -> HashSet<WireName>
where
    F: Fn(usize, usize) -> usize,
{
    let cache = &mut HashSet::new();
    xy_inputs
        .iter()
        .flat_map(|(x, y)| get_wrong_bits(&wire_state_map, &expected_operation, *x, *y))
        .flat_map(|wire| get_all_wires_involved_for_wire(&wire_state_map, &wire, cache))
        .collect()
}

fn get_wrong_bits<F>(
    wire_state_map: &ParsedInput,
    expected_operation: F,
    x: usize,
    y: usize,
) -> Vec<WireName>
where
    F: Fn(usize, usize) -> usize,
{
    let mut wire_state_map_input_set = wire_state_map.clone();
    set_input(&mut wire_state_map_input_set, "x", x);
    set_input(&mut wire_state_map_input_set, "y", y);

    let zs = wire_state_map_input_set
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .collect_vec();

    let expected_result = expected_operation(x, y);

    let zs_binary = zs
        .clone()
        .into_iter()
        .rev()
        .map(|z| get_bit_value(&wire_state_map_input_set, z))
        .map(|bit| if bit { "1" } else { "0" })
        .join("");

    let current_result = usize::from_str_radix(zs_binary.as_str(), 2).unwrap();

    let bit_diff = format!("{:0>46b}", expected_result.bitxor(current_result))
        .chars()
        .rev()
        .zip(zs)
        .filter_map(|(bit_value, z)| if bit_value == '1' { Some(z) } else { None })
        .cloned()
        .collect_vec();

    bit_diff
}

fn set_input(wire_state_map: &mut ParsedInput, wire_name: &str, value: usize) {
    let wire_bits = wire_state_map
        .keys()
        .filter(|key| key.starts_with(wire_name))
        .sorted()
        .cloned()
        .collect_vec();

    let string = format!("{:0>45b}", value);
    let wire_bits_value = string.chars().rev().zip(wire_bits).collect_vec();
    wire_bits_value
        .into_iter()
        .for_each(|(bit_value, bit_name)| {
            let wire_state = match bit_value {
                '0' => ZERO,
                '1' => ONE,
                _ => unreachable!(),
            };
            wire_state_map.insert(bit_name, wire_state);
        })
}

fn get_bit_value(wire_state_map: &ParsedInput, wire_name: &WireName) -> bool {
    let wire_state = wire_state_map.get(wire_name).unwrap();

    match wire_state {
        AND(left_wire, right_wire) => get_bit_value(wire_state_map, left_wire)
            .bitand(get_bit_value(wire_state_map, right_wire)),
        OR(left_wire, right_wire) => get_bit_value(wire_state_map, left_wire)
            .bitor(get_bit_value(wire_state_map, right_wire)),
        XOR(left_wire, right_wire) => get_bit_value(wire_state_map, left_wire)
            .bitxor(get_bit_value(wire_state_map, right_wire)),
        ONE => true,
        ZERO => false,
    }
}

fn get_all_wires_involved_for_wire(
    wire_state_map: &ParsedInput,
    wire_name: &WireName,
    current_wire_set: &mut HashSet<WireName>,
) -> HashSet<WireName> {
    if current_wire_set.contains(wire_name) {
        return current_wire_set.clone();
    }

    let wire_state = wire_state_map.get(wire_name).unwrap();

    let rec_set: HashSet<WireName> = match wire_state {
        AND(left_wire, right_wire) | OR(left_wire, right_wire) | XOR(left_wire, right_wire) => {
            get_all_wires_involved_for_wire(wire_state_map, left_wire, current_wire_set)
                .union(&get_all_wires_involved_for_wire(
                    wire_state_map,
                    right_wire,
                    current_wire_set,
                ))
                .cloned()
                .collect()
        }
        ONE | ZERO => HashSet::new(),
    };

    let new_set: HashSet<_> = rec_set
        .union(&HashSet::from([wire_name.clone()]))
        .cloned()
        .collect();
    current_wire_set.extend(new_set);
    current_wire_set.clone()
}

#[cfg(test)]
mod test {
    use crate::year2024::day24::lib::part2::part2;
    use crate::year2024::day24::lib::YEAR_2024_DAY_24_SOLUTION;
    use itertools::{iterate, Itertools};
    use std::ops::BitAnd;

    #[test]
    fn part1_input() {
        assert_eq!(
            part2(
                YEAR_2024_DAY_24_SOLUTION.get_parsed_inputs(),
                |a, b| a + b,
                iterate((0usize, 1usize), |(x, y)| (*x, y << 1))
                    .take(45)
                    .flat_map(|(x, y)| [(x, y), (y, x)])
                    .collect_vec(),
            ),
            "z00,z05,z01,z02"
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                YEAR_2024_DAY_24_SOLUTION.get_parsed_test_inputs(2),
                |a, b| a.bitand(b),
                vec![
                    (0b00001usize, 0b00001usize),
                    (0b00010, 0b00010),
                    (0b00100, 0b00100),
                    (0b01000, 0b01000),
                    (0b10000, 0b10000),
                ]
            ),
            "z00,z01,z02,z05"
        );
    }
}
