use std::collections::HashSet;
use itertools::Itertools;
use crate::year2024::day05::lib::parser::{Rule, UpdateLine};

pub mod parser;

pub fn is_valid(rules: &HashSet<Rule>, update_line: &UpdateLine) -> bool {
    update_line.iter()
        .combinations(2)
        .all(|update_combination| !rules.contains(&(*update_combination[1], *update_combination[0])))
}
