use crate::year2024::day24::lib::parser::WireState::{ONE, ZERO};
use crate::year2024::day24::lib::parser::{ParsedInput, WireName, WireState};
use itertools::Itertools;
use std::collections::HashMap;
use WireState::{AND, OR, XOR};

// Using the full binary adder logic https://www.electronics-tutorials.ws/combination/comb_7.html
// We can deduce where the outputs should be connected to
pub fn part2(wire_state_map: ParsedInput) -> String {
    let wire_state_from_input_map = wire_state_map
        .clone()
        .into_iter()
        .flat_map(|(output_wire, gate)| match gate {
            AND(left, right) | OR(left, right) | XOR(left, right) => {
                vec![(left, output_wire.clone()), (right, output_wire)]
            }
            ONE | ZERO => vec![],
        })
        .into_group_map();

    let gate_outputs_to_swap = wire_state_map
        .clone()
        .into_iter()
        .flat_map(|(wire_name, wire_state)| {
            if wire_name.starts_with("z") {
                match wire_state {
                    XOR(_, _) => vec![],
                    OR(_, _) => {
                        if wire_name != "z45" {
                            vec![wire_name]
                        } else {
                            vec![]
                        }
                    }
                    _ => vec![wire_name],
                }
            } else {
                match wire_state.clone() {
                    AND(_, _) => {
                        if wire_state.has_xy_inputs() {
                            let connects_to = get_connects_to_gates(
                                &wire_state_map,
                                &wire_state_from_input_map,
                                &wire_name,
                            );

                            if connects_to.len() == 2 && wire_state.has_initial_xy_inputs() {
                                vec![]
                            } else if connects_to.len() != 1 {
                                vec![wire_name]
                            } else if connects_to[0].is_or()
                                || connects_to[0].is_xor() && wire_state.has_initial_xy_inputs()
                            {
                                vec![]
                            } else {
                                vec![wire_name]
                            }
                        } else {
                            let connects_to = get_connects_to_gates(
                                &wire_state_map,
                                &wire_state_from_input_map,
                                &wire_name,
                            );

                            if connects_to.len() != 1 {
                                vec![wire_name]
                            } else if connects_to[0].is_or() {
                                vec![]
                            } else {
                                vec![wire_name]
                            }
                        }
                    }
                    XOR(_, _) => {
                        if wire_state.has_xy_inputs() {
                            let connects_to = get_connects_to_gates(
                                &wire_state_map,
                                &wire_state_from_input_map,
                                &wire_name,
                            );

                            if connects_to.len() != 2 {
                                vec![wire_name]
                            } else if connects_to[0].is_and() && connects_to[1].is_xor()
                                || connects_to[1].is_and() && connects_to[0].is_xor()
                            {
                                vec![]
                            } else {
                                vec![wire_name]
                            }
                        } else {
                            vec![wire_name]
                        }
                    }
                    OR(left, right) => {
                        let mut result = vec![];
                        if !wire_state_map.get(&left).unwrap().is_and() {
                            result.push(left);
                        }

                        if !wire_state_map.get(&right).unwrap().is_and() {
                            result.push(right);
                        }
                        result
                    }
                    _ => vec![],
                }
            }
        })
        .collect_vec();

    gate_outputs_to_swap.iter().sorted().unique().join(",")
}

fn get_connects_to_gates(
    wire_state_map: &ParsedInput,
    wire_state_from_input_map: &HashMap<WireName, Vec<String>>,
    wire_name: &WireName,
) -> Vec<WireState> {
    wire_state_from_input_map
        .get(wire_name)
        .unwrap()
        .into_iter()
        .map(|wire_gate| wire_state_map.get(wire_gate).unwrap())
        .cloned()
        .collect_vec()
}
