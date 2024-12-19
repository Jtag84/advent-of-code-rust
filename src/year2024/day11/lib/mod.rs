use crate::year2024::day11::lib::parser::Stone;
use std::collections::HashMap;

pub mod parser;

pub fn blink_n(stones: Vec<Stone>, n: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| apply_rules_n(*stone, n, &mut cache))
        .sum()
}

fn apply_rules_n(stone: Stone, n: usize, cache: &mut HashMap<(Stone, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }

    let cache_key = &(stone, n);
    match cache.get(cache_key) {
        Some(number_of_stones) => return *number_of_stones,
        None => {}
    }

    let stones_after_rule = if stone == 0 {
        vec![1]
    } else {
        let stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let (left_stone_string, right_stone_string) =
                stone_string.split_at(stone_string.len() / 2);
            vec![
                left_stone_string.parse().unwrap(),
                right_stone_string.parse().unwrap(),
            ]
        } else {
            vec![stone * 2024]
        }
    };

    let number_of_stones: usize = stones_after_rule
        .iter()
        .map(|stone| apply_rules_n(*stone, n - 1, cache))
        .sum();
    cache.insert(*cache_key, number_of_stones);
    number_of_stones
}
