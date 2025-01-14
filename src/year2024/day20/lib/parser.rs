use crate::lib::grid_utils::{set_grid_element, GridCoordinates};
use crate::lib::parser_commons::parse_grid;
use grid::Grid;

pub type Racetrack = Grid<char>;
pub type Start = GridCoordinates;
pub type End = GridCoordinates;

pub type ParsedInput = (Racetrack, Start, End);

pub fn parse_input(input_path: &str) -> ParsedInput {
    let mut racetrack = parse_grid(&input_path);
    let start = racetrack
        .indexed_iter()
        .find(|(_, v)| **v == 'S')
        .map(|(c, _)| GridCoordinates::from(c))
        .unwrap();
    let end = racetrack
        .indexed_iter()
        .find(|(_, v)| **v == 'E')
        .map(|(c, _)| GridCoordinates::from(c))
        .unwrap();

    set_grid_element(&mut racetrack, &start, '.');
    set_grid_element(&mut racetrack, &end, '.');

    (racetrack, start, end)
}

#[cfg(test)]
mod test {
    use crate::lib::grid_utils::grid_to_str;
    use crate::year2024::day20::lib::YEAR_2024_DAY_20_SOLUTION;

    #[test]
    fn test_parser_part1() {
        let (racetrack, start, end) = YEAR_2024_DAY_20_SOLUTION.get_parsed_test_inputs(1);
        println!("start: {start:?}, end: {end:?}");
        println!("{}", grid_to_str(&racetrack));
    }

    #[test]
    fn test_parser_part2() {
        let (racetrack, start, end) = YEAR_2024_DAY_20_SOLUTION.get_parsed_test_inputs(2);
        println!("start: {start:?}, end: {end:?}");
        println!("{}", grid_to_str(&racetrack));
    }
}
