use itertools::{iterate, Itertools};
use std::ops::{BitXor, Div, Mul, Rem};

pub mod parser;

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
