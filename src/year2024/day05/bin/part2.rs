use adv_code::year2024::day05::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use adv_code::year2024::day05::lib::is_valid;

const INPUT_FILE: &str = "input/2024/05/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 05, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 4713);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let (rules, updates) = parse_input(&file_input_path);

    let mut updates: Vec<_> = updates.into_iter()
        .filter(|update_line| !is_valid(&rules, update_line))
        .collect();

    for update_line in updates.iter_mut() {
        let mut index = 0;
        'update_line_loop: while index < update_line.len() {
            let current_update = *update_line.get(index).expect(&format!("error {index} in {update_line:?}"));
            for (next_index_offset, next_update) in update_line[(index + 1)..].iter().enumerate() {
                if rules.contains(&(*next_update, current_update)) {
                    update_line.swap(index, index + 1 + next_index_offset);
                    continue 'update_line_loop;
                }
            }
            index += 1;
        }
    }

    updates.into_iter()
        .map(|update_line| *update_line.get(update_line.len() / 2).expect("error"))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part2;

    const TEST_INPUT_FILE: &str = "input/2024/05/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 123);
    }
}