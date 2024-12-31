use crate::lib::grid_utils::{Direction, GridCoordinates};
use crate::lib::parser_commons::{parse_grid_from_string, read_file_to_string};
use grid::Grid;
use itertools::Itertools;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::many1;
use nom::IResult;

pub type RobotPosition = GridCoordinates;

fn parse_map(input: String) -> (Grid<char>, RobotPosition) {
    let mut grid = parse_grid_from_string(input);
    let robot_pos = grid
        .indexed_iter_mut()
        .find_map(|(pos, value)| {
            if *value == '@' {
                *value = '.';
                Some(GridCoordinates::from(pos))
            } else {
                None
            }
        })
        .expect("cannot find robot position");

    (grid, robot_pos)
}

type Movements = Vec<Direction>;

fn parse_movement(input: &str) -> IResult<&str, Direction> {
    let (input, movement) = map(one_of("<^>v"), |c| match c {
        '<' => Direction::West,
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        _ => panic!("unexpected char {}", c),
    })(input)?;

    Ok((input, movement))
}

fn parse_movements(input: &str) -> IResult<&str, Movements> {
    let (input, movements) = many1(parse_movement)(input)?;
    Ok((input, movements))
}

pub fn parse_input(input_path: &str) -> (Grid<char>, RobotPosition, Movements) {
    let file_string = read_file_to_string(input_path);

    let split: Vec<_> = file_string.split_terminator("\n\n").collect();

    let (grid, robot_position) = parse_map(split[0].to_string());

    let movements_string_joined = split[1].lines().join("");
    let movements_res = parse_movements(movements_string_joined.as_str());

    (
        grid,
        robot_position,
        movements_res.expect("parsing error").1,
    )
}
