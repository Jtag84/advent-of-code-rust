use std::fs::File;
use std::io::Read;

pub mod year2024;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
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
        start_day("00");
    }
}
