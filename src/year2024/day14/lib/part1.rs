use crate::lib::grid_utils::XYCoordinates;
use crate::year2024::day14::lib::calculate_robots_at_n_seconds;
use crate::year2024::day14::lib::parser::ParsedInput;

pub fn part1(robots: ParsedInput, map_width: isize, map_height: isize) -> String {
    let width_middle = map_width / 2;
    let height_middle = map_height / 2;
    let robots_after_100s = calculate_robots_at_n_seconds(map_width, map_height, &robots, 100);

    let (left_half, right_half): (Vec<XYCoordinates>, Vec<XYCoordinates>) = robots_after_100s
        .iter()
        .filter(|XYCoordinates(x, y)| *x != width_middle && *y != height_middle)
        .partition(|XYCoordinates(x, _)| *x < width_middle);

    let quadrants = [left_half, right_half]
        .into_iter()
        .map(|half| {
            half.iter()
                .partition(|XYCoordinates(_, y)| *y < height_middle)
        })
        .collect::<Vec<_>>();

    quadrants
        .into_iter()
        .map(
            |(up_half, down_half): (Vec<XYCoordinates>, Vec<XYCoordinates>)| {
                up_half.len() * down_half.len()
            },
        )
        .reduce(|accum, item| accum * item)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day14::lib::{part1, YEAR_2024_DAY_14_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_14_SOLUTION.get_parsed_test_inputs(1), 11, 7),
            "12"
        );
    }
}
