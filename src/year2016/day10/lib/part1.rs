use crate::year2016::day10::lib::parser::ValueDestination::{ToBot, ToOutputBin};
use crate::year2016::day10::lib::parser::{Bot, BotId, Bots, OutputBins, ParsedInput, Value};
use itertools::Itertools;
use std::cmp::{max, min};

pub fn part1(
    (mut bots_map, mut output_map, initial_values): ParsedInput,
    find_bot_comparing_values: Vec<Value>,
) -> String {
    initial_values
        .iter()
        .filter_map(|(bot_id, microchip_id)| {
            handle_microchip_value_and_find(
                *bot_id,
                *microchip_id,
                &mut bots_map,
                &mut output_map,
                &|bot_id: &BotId, Bot { value, .. }, microchip_id| {
                    if let Some(holding_value) = value {
                        if find_bot_comparing_values.contains(&holding_value)
                            && find_bot_comparing_values.contains(microchip_id)
                        {
                            Some(bot_id.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
            )
        })
        .take(1)
        .exactly_one()
        .unwrap()
        .to_string()
}

pub fn handle_microchip_value_and_find<R>(
    bot_id: BotId,
    microchip_id: Value,
    bots_map: &mut Bots,
    output_map: &mut OutputBins,
    predicate: &dyn Fn(&BotId, &Bot, &Value) -> Option<R>,
) -> Option<R> {
    let bot = bots_map.get_mut(&bot_id).unwrap();

    let predicate_result = predicate(&bot_id, &bot, &microchip_id);
    if predicate_result.is_some() {
        return predicate_result;
    }

    if let Some(holding_value) = bot.value {
        bot.value = None;
        let gives_low_to = bot.gives_low_to.clone();
        let gives_high_to = bot.gives_high_to.clone();
        let low = min(holding_value, microchip_id);
        let high = max(holding_value, microchip_id);
        match gives_low_to {
            ToBot(to_bot_id) => {
                handle_microchip_value_and_find(to_bot_id, low, bots_map, output_map, predicate)
            }
            ToOutputBin(to_output_bin_id) => {
                output_map
                    .get_mut(&to_output_bin_id)
                    .unwrap()
                    .values
                    .push(low);
                None
            }
        }
        .or_else(|| match gives_high_to {
            ToBot(to_bot_id) => {
                handle_microchip_value_and_find(to_bot_id, high, bots_map, output_map, predicate)
            }
            ToOutputBin(to_output_bin_id) => {
                output_map
                    .get_mut(&to_output_bin_id)
                    .unwrap()
                    .values
                    .push(high);
                None
            }
        })
    } else {
        bot.value = Some(microchip_id);
        None
    }
}

#[cfg(test)]
mod test {
    use crate::year2016::day10::lib::part1::part1;
    use crate::year2016::day10::lib::YEAR_2016_DAY_10_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                YEAR_2016_DAY_10_SOLUTION.get_parsed_test_inputs(1),
                vec![5, 2]
            ),
            "2"
        );
    }
}
