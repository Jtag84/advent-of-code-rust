use crate::year2024::day12::lib::group_gardens_for_all;
use crate::year2024::day12::lib::parser::ParsedInput;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(grid: ParsedInput) -> String {
    let mut garden_map = HashMap::new();
    group_gardens_for_all(&mut garden_map, &grid);

    garden_map
        .values()
        .into_iter()
        .map(|g| g.get_root_garden_struct())
        .unique_by(|g| g.id())
        .map(|garden| garden.area() * garden.perimeter())
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day12::lib::part1::part1;
    use crate::year2024::day12::lib::YEAR_2024_DAY_12_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_12_SOLUTION.get_parsed_test_inputs(1)),
            "1930"
        );
    }
}
