use std::fs::File;
use std::io::Read;

pub mod year2024;

pub fn start_day(year: i32, day: i32, part: i32) {
    println!("Advent of Code {year} - Day {:0>2}", day);
    println!("=== Part {part} ===");
}

pub fn read_file_to_string(input_path: &str) -> String {
    let mut file_string = String::new();
    File::open(input_path).expect("Can't open file")
        .read_to_string(&mut file_string).expect("can't read file");
    file_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day(2024, 0, 0);
    }
}
