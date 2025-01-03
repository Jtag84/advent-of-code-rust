use adv_code::year2024::day19::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::{error, Compare, IResult, InputLength, InputTake};
use std::collections::HashMap;
use std::hash::Hash;

const INPUT_FILE: &str = "input/2024/19/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 19, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 600639829400603);

    Ok(())
}

fn part2(file_input_path: &str) -> isize {
    let (patterns, designs) = parse_input(&file_input_path);
    let patterns_str = patterns.iter().map(|s| s.as_str()).collect_vec();
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| {
            parse_design_match_count(patterns_str.as_slice(), &mut cache)(design)
                .unwrap_or(("", 0))
                .1
        })
        .sum()
}

pub fn parse_design_match_count<'a, Input>(
    patterns: &'a [Input],
    cache: &'a mut HashMap<Input, isize>,
) -> impl FnMut(Input) -> IResult<Input, isize> + 'a
where
    Input: InputTake + Compare<Input> + InputLength + Clone + Hash + Eq,
{
    move |input: Input| {
        if let Some(count) = cache.get(&input) {
            return std::prelude::rust_2015::Ok((input, *count));
        }

        let mut matched_count = 0;
        for pattern in patterns {
            let res = tag::<Input, Input, error::Error<Input>>(pattern.clone())(input.clone());
            if let std::prelude::rust_2015::Ok((remaining, _)) = res {
                if remaining.input_len() == 0 {
                    matched_count += 1;
                } else {
                    let rec_res = parse_design_match_count::<Input>(patterns, cache)(remaining);
                    if let std::prelude::rust_2015::Ok((_, count)) = rec_res {
                        matched_count += count;
                    }
                }
            }
        }
        cache.insert(input.clone(), matched_count);
        std::prelude::rust_2015::Ok((input, matched_count))
    }
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day19::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/19/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 16);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
