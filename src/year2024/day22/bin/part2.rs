use adv_code::year2024::day22::lib::parser::parse_input;
use adv_code::year2024::day22::lib::pseudorandom;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::{iterate, Itertools};
use std::ops::Rem;

const INPUT_FILE: &str = "input/2024/22/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 22, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 1455);

    Ok(())
}

fn part2(file_input_path: &str) -> u16 {
    let secrets = parse_input(&file_input_path);
    let price_changes_sequence_map = secrets
        .into_iter()
        .flat_map(price_changes_sequences)
        .into_group_map();

    price_changes_sequence_map
        .values()
        .map(|prices| prices.iter().sum())
        .max()
        .unwrap()
}

fn get_prices(secret: usize) -> Vec<Price> {
    iterate(secret, |s| pseudorandom(*s))
        .map(|s| s.rem(10) as Price)
        .take(2000)
        .collect()
}

type PriceChangeSequence = (i8, i8, i8, i8);
type Price = u16;

fn price_changes_sequences(secret: usize) -> Vec<(PriceChangeSequence, Price)> {
    vec![0]
        .into_iter()
        .chain(get_prices(secret))
        .tuple_windows::<(_, _, _, _, _)>()
        .map(|(a, b, c, d, e)| {
            (
                (
                    b as i8 - a as i8,
                    c as i8 - b as i8,
                    d as i8 - c as i8,
                    e as i8 - d as i8,
                ),
                e,
            )
        })
        .unique_by(|(price_change_sequence, _)| *price_change_sequence)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{get_prices, part2, price_changes_sequences};
    use adv_code::year2024::day22::lib::parser::parse_input;
    use itertools::Itertools;

    const TEST_INPUT_FILE: &str = "input/2024/22/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 23);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn to_prices_test() {
        let prices = get_prices(123).into_iter().take(10).collect_vec();
        println!("{:?}", prices);
        assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn price_changes_sequence_map_test() {
        let price_changes_sequences = price_changes_sequences(123);
        println!("price_changes_sequences {:?}", price_changes_sequences);
    }
}
