use crate::year2024::day22::lib::pseudorandom_n;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

pub fn part1(secrets: Vec<usize>) -> String {
    secrets
        .into_par_iter()
        .map(|secret| pseudorandom_n(secret, 2000))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day22::lib::part1::part1;
    use crate::year2024::day22::lib::{pseudorandom, pseudorandom_n, DAY_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(part1(DAY_SOLUTION.get_parsed_test_inputs(1)), "37327623");
    }

    #[test]
    fn test_pseudorandom() {
        let next_secret = pseudorandom(123);
        assert_eq!(next_secret, 15887950);
    }

    #[test]
    fn test_pseudorandom_n() {
        let next_secret = pseudorandom_n(123, 10);
        assert_eq!(next_secret, 5908254);
    }
}
