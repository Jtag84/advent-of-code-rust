use crate::year2024::day19::lib::parser::ParsedInput;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::{error, Compare, IResult, InputLength, InputTake};

pub fn part1((patterns, designs): ParsedInput) -> String {
    let patterns_str = patterns.iter().map(|s| s.as_str()).collect_vec();
    designs
        .iter()
        .filter(|design| {
            let result = parse_design(patterns_str.as_slice())(design);
            result.is_ok()
        })
        .count()
        .to_string()
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
            if let Ok((remaining, matched)) = res {
                if remaining.input_len() == 0 {
                    return Ok((remaining, matched));
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
    use crate::year2024::day19::lib::part1;
    use crate::year2024::day19::lib::YEAR_2024_DAY_19_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_19_SOLUTION.get_parsed_test_inputs(1)),
            "6"
        );
    }
}
