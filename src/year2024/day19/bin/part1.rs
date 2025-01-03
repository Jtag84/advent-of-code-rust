use adv_code::year2024::day19::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::{error, Compare, IResult, InputLength, InputTake};

const INPUT_FILE: &str = "input/2024/19/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 19, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 358);

    Ok(())
}

fn part1(file_input_path: &str) -> isize {
    let (patterns, designs) = parse_input(&file_input_path);
    let patterns_str = patterns.iter().map(|s| s.as_str()).collect_vec();
    designs
        .iter()
        .filter(|design| {
            let result = parse_design(patterns_str.as_slice())(design);
            result.is_ok()
        })
        .count() as isize
}

pub fn parse_design<'a, Input>(
    patterns: &'a [Input],
) -> impl Fn(Input) -> IResult<Input, Input> + 'a
where
    Input: InputTake + Compare<Input> + InputLength + Clone,
{
    move |input: Input| {
        for pattern in patterns {
            let res = tag::<Input, Input, error::Error<Input>>(pattern.clone())(input.clone());
            if let std::prelude::rust_2015::Ok((remaining, matched)) = res {
                if remaining.input_len() == 0 {
                    return std::prelude::rust_2015::Ok((remaining, matched));
                } else {
                    let rec_res = parse_design::<Input>(patterns)(remaining);
                    if rec_res.is_ok() {
                        return rec_res;
                    }
                }
            }
        }
        Err(nom::Err::Error(error::Error::new(input, ErrorKind::Alt)))
    }
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day19::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/19/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 6);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
