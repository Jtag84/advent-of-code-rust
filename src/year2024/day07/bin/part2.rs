use adv_code::year2024::day07::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use std::collections::HashMap;

const INPUT_FILE: &str = "input/2024/07/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 07, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 581941094529163);

    Ok(())
}

fn part2(file_input_path: &str) -> isize {
    let equations = parse_input(&file_input_path);

    equations
        .into_iter()
        .map(|(result, numbers)| {
            let mut cache = HashMap::new();
            let remaining_numbers = numbers.as_slice();
            apply_operators_for_alternative_rec(&remaining_numbers[0..], &mut cache)
                .into_iter()
                .find(|output| result == *output)
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .sum()
}

pub fn apply_operators_for_alternative_rec<'a, 'b>(
    numbers: &'a [isize],
    cache: &'b mut HashMap<&'a [isize], Vec<isize>>,
) -> Vec<isize> {
    let cache_key = numbers;
    if let Some(result) = cache.get(cache_key) {
        return result.clone();
    }

    if numbers.len() == 1 {
        return vec![numbers[0]];
    }

    let last_index = numbers.len() - 1;
    let last_number = numbers[last_index];
    let remaining_numbers: &[isize] = numbers[0..last_index].try_into().unwrap();

    let result: Vec<isize> = vec![
        apply_operators_for_alternative_rec(remaining_numbers, cache)
            .iter()
            .map(|total| total + &last_number)
            .collect::<Vec<_>>(),
        apply_operators_for_alternative_rec(remaining_numbers, cache)
            .iter()
            .map(|total| total * &last_number)
            .collect(),
        apply_operators_for_alternative_rec(remaining_numbers, cache)
            .iter()
            .map(|total| format!("{total}{last_number}").parse().unwrap())
            .collect(),
    ]
    .into_iter()
    .flatten()
    .collect();

    cache.insert(cache_key, result.clone());
    result
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day07::lib::parser::parse_input;
    use code_timing_macros::time_snippet;

    const TEST_INPUT_FILE: &str = "input/2024/07/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 11387);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_concat() {
        let result: isize = time_snippet!({
            let left = 123;
            let right = 456;
            let result_string = format!("{left}{right}");
            result_string.parse().unwrap()
        });

        assert_eq!(result, 123456);
    }
}
