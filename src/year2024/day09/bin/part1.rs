use adv_code::year2024::day09::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/09/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 09, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 6435922584968);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let disk_layout = parse_input(&file_input_path);
    let mut file_compacted_map: Vec<usize> = Vec::new();

    let mut left_id: usize = 0;
    let mut right_id = disk_layout.len() / 2;
    let mut right_index = disk_layout.len() - 1;
    let mut right_number_remaining = disk_layout.get(right_index).unwrap().clone();
    for (left_index, layout_element) in disk_layout.iter().enumerate() {
        if right_index <= left_index {
            break;
        }

        if left_index % 2 == 0 {
            (0_usize..*layout_element).for_each(|_| {
                file_compacted_map.push(left_id);
            });
            left_id += 1;
        } else {
            (0_usize..*layout_element).for_each(|_| {
                if right_number_remaining <= 0 {
                    right_index -= 2;
                    right_id -= 1;
                    right_number_remaining = disk_layout.get(right_index).unwrap().clone();
                }
                file_compacted_map.push(right_id);
                right_number_remaining -= 1;
            });
        }
    }
    if right_number_remaining > 0 && left_id <= right_id {
        (0..right_number_remaining).for_each(|_| file_compacted_map.push(right_id));
    }
    file_compacted_map
        .into_iter()
        .enumerate()
        .map(|(index, id)| index * id)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part1;
    use adv_code::year2024::day09::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/09/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 1928);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
