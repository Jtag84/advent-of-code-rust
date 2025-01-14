pub use crate::aoc_solver;
pub use crate::year2024::day22::lib::parser::parse_input;
use crate::year2024::day22::lib::parser::ParsedInput;
pub use crate::year2024::day22::lib::part1::part1;
pub use crate::year2024::day22::lib::part2::part2;
pub use crate::DaySolution;
pub use crate::SOLUTIONS;
use itertools::{iterate, Itertools};
use std::ops::{BitXor, Div, Mul, Rem};

pub mod parser;
pub mod part1;
pub mod part2;

const YEAR_2024_DAY_22_SOLUTION: DaySolution<ParsedInput> = DaySolution {
    year_day: (2024, 22),
    parser: parse_input,
    part1,
    expected_part1: "13004408787",
    part2,
    expected_part2: "1455",
};

aoc_solver!(YEAR_2024_DAY_22_SOLUTION);

pub fn pseudorandom_n(secret: usize, n: usize) -> usize {
    iterate(secret, |s| pseudorandom(*s))
        .skip(n)
        .take(1)
        .exactly_one()
        .expect("Expected a number")
}

pub fn pseudorandom(mut secret: usize) -> usize {
    secret = secret.mul(64).bitxor(secret).rem(16777216);
    secret = secret.div(32).bitxor(secret).rem(16777216);
    secret = secret.mul(2048).bitxor(secret).rem(16777216);
    secret
}
