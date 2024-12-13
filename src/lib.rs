pub mod year2024;

pub mod lib{
    pub mod grid_utils;
    pub mod parser_commons;
}

pub fn start_day(year: i32, day: i32, part: i32) {
    println!("Advent of Code {year} - Day {:0>2}", day);
    println!("=== Part {part} ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day(2024, 0, 0);
    }
}
