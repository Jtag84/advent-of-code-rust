use adv_code::year2024::day07::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/07/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 07, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 21572148763543);

    Ok(())
}

fn part1(file_input_path: &str) -> isize {
    let equations = parse_input(&file_input_path);

    equations
        .into_iter()
        .map(|(result, numbers)| {
            let number_of_operators = numbers.len();
            (0..2_isize.pow(number_of_operators.try_into().unwrap()))
                .map(|alternative_index| {
                    apply_operators_for_alternative_i(&numbers, alternative_index)
                })
                .find(|output| result == *output)
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .sum()
}

pub fn apply_operators_for_alternative_i(numbers: &Vec<isize>, alternative_index: isize) -> isize {
    let mut total = numbers[0];
    for (index, number) in numbers.into_iter().skip(1).enumerate() {
        let bit_mask_multiply = 1 << index;
        if alternative_index & bit_mask_multiply != 0 {
            // multiply operator
            total *= number;
        } else {
            // add operator
            total += number;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use crate::{apply_operators_for_alternative_i, part1};
    use adv_code::year2024::day07::lib::parser::parse_input;
    use itertools::{repeat_n, Itertools};

    const TEST_INPUT_FILE: &str = "input/2024/07/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 3749);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_cartesian_product() {
        let cartesian_product = (0..3).cartesian_product(4..7).collect::<Vec<_>>();
        println!("{:?}", cartesian_product);
    }

    #[test]
    fn test_permutations() {
        let mut list_1_and_0s = repeat_n(0, 3).collect::<Vec<_>>();
        list_1_and_0s.append(repeat_n(1, 3).collect::<Vec<_>>().as_mut());

        println!(
            "{:?}",
            list_1_and_0s
                .iter()
                .permutations(3)
                .unique()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_apply_operator() {
        let numbers: Vec<_> = (0..6).collect();
        let alternative_index = 3;
        let total = apply_operators_for_alternative_i(&numbers, alternative_index);
        println!("{total}");
    }
}
