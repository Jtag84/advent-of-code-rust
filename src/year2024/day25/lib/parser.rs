use crate::lib::parser_commons::{parse_grid_from_string_with, read_file_to_string};
use grid::Grid;
use itertools::{Either, Itertools};

pub type Lock = Grid<char>;
pub type Key = Grid<char>;
pub type ParsedInput = (Vec<Lock>, Vec<Key>);

pub fn parse_input(input_path: &str) -> ParsedInput {
    let file_string = read_file_to_string(input_path);
    let locks_and_keys = file_string
        .lines()
        .chunks(8)
        .into_iter()
        .map(|mut chunk| chunk.join("\n").to_string())
        .map(|chunk| parse_grid_from_string_with(chunk, |c| c.try_into().unwrap()))
        .collect_vec();

    locks_and_keys.into_iter().partition_map(|lk| {
        if *lk.get(0, 0).unwrap() == '#' {
            Either::Left(lk)
        } else {
            Either::Right(lk)
        }
    })
}

#[cfg(test)]
mod test {
    use crate::lib::grid_utils::grid_to_str;
    use crate::year2024::day25::lib::YEAR_2024_DAY_25_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let (locks, keys) = YEAR_2024_DAY_25_SOLUTION.get_parsed_test_inputs(1);
        println!("Locks:");
        locks
            .iter()
            .for_each(|lock| println!("{:}", grid_to_str(lock)));
        println!("Keys:");
        keys.iter()
            .for_each(|key| println!("{:}", grid_to_str(key)));
    }
}
