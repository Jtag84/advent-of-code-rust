use crate::year2024::day07::lib::parser::ParsedInput;
use std::collections::HashMap;

pub fn part2(equations: ParsedInput) -> String {
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
        .sum::<isize>()
        .to_string()
}

pub fn apply_operators_for_alternative_rec<'a>(
    numbers: &'a [isize],
    cache: &mut HashMap<&'a [isize], Vec<isize>>,
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
    use crate::year2024::day07::lib::part2::part2;
    use crate::year2024::day07::lib::YEAR_2024_DAY_07_SOLUTION;

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_07_SOLUTION.get_parsed_test_inputs(2)),
            "11387"
        );
    }
}
