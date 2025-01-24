use crate::year2016::day10::lib::parser::{BotId, ParsedInput};
use crate::year2016::day10::lib::part1::handle_microchip_value_and_find;

pub fn part2((mut bots_map, mut output_map, initial_values): ParsedInput) -> String {
    initial_values.iter().for_each(|(bot_id, microchip_id)| {
        handle_microchip_value_and_find::<BotId>(
            *bot_id,
            *microchip_id,
            &mut bots_map,
            &mut output_map,
            &|_, _, _| None,
        );
    });

    let result = output_map.get(&0).unwrap().values[0]
        * output_map.get(&1).unwrap().values[0]
        * output_map.get(&2).unwrap().values[0];
    result.to_string()
}

#[cfg(test)]
mod test {
    use crate::year2016::day10::lib::part2::part2;
    use crate::year2016::day10::lib::YEAR_2016_DAY_10_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2016_DAY_10_SOLUTION.get_parsed_test_inputs(2)),
            "30"
        );
    }
}
