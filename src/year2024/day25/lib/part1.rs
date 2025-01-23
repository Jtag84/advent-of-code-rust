use crate::year2024::day25::lib::parser::ParsedInput;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub fn part1((locks, keys): ParsedInput) -> String {
    let locks_max_rows = locks
        .into_iter()
        .map(|lock| {
            lock.iter_cols()
                .map(|c| {
                    c.enumerate()
                        .filter(|(_, c)| **c == '#')
                        .map(|(index, _)| index)
                        .max()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let keys_min_rows = keys
        .into_iter()
        .map(|key| {
            key.iter_cols()
                .map(|c| {
                    6 - c
                        .enumerate()
                        .filter(|(_, c)| **c == '#')
                        .map(|(index, _)| index)
                        .min()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    locks_max_rows
        .into_par_iter()
        .map(|lock| {
            keys_min_rows
                .iter()
                .filter(|key| lock.iter().zip(key.iter()).all(|(a, b)| a + b < 6))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day25::lib::part1::part1;
    use crate::year2024::day25::lib::YEAR_2024_DAY_25_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_25_SOLUTION.get_parsed_test_inputs(1)),
            "3"
        );
    }
}
