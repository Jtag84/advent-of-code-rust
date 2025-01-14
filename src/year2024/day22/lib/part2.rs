use crate::year2024::day22::lib::parser::ParsedInput;
use crate::year2024::day22::lib::pseudorandom;
use itertools::{iterate, Itertools};
use rayon::prelude::*;
use std::ops::Rem;

pub fn part2(secrets: ParsedInput) -> String {
    let price_changes_sequence_map = secrets
        .into_par_iter()
        .flat_map(price_changes_sequences)
        .collect::<Vec<_>>()
        .into_iter()
        .into_group_map();

    price_changes_sequence_map
        .values()
        .map(|prices| prices.iter().sum::<Price>())
        .max()
        .unwrap()
        .to_string()
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
    use crate::year2024::day22::lib::part2::{get_prices, part2, price_changes_sequences};
    use crate::year2024::day22::lib::YEAR_2024_DAY_22_SOLUTION;
    use itertools::Itertools;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_22_SOLUTION.get_parsed_test_inputs(2)),
            "23"
        );
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
