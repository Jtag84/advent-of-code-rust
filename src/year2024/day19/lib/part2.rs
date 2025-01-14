use crate::year2024::day19::lib::parser::ParsedInput;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::{error, Compare, IResult, InputLength, InputTake};
use std::collections::HashMap;
use std::hash::Hash;

pub fn part2((patterns, designs): ParsedInput) -> String {
    let patterns_str = patterns.iter().map(|s| s.as_str()).collect_vec();
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| {
            parse_design_match_count(patterns_str.as_slice(), &mut cache)(design)
                .unwrap_or(("", 0))
                .1
        })
        .sum::<isize>()
        .to_string()
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
    use crate::year2024::day19::lib::part2::part2;
    use crate::year2024::day19::lib::YEAR_2024_DAY_19_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_19_SOLUTION.get_parsed_test_inputs(2)),
            "16"
        );
    }
}
