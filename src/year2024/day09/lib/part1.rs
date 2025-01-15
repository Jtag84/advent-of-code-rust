use crate::year2024::day09::lib::parser::ParsedInput;

pub fn part1(disk_layout: ParsedInput) -> String {
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
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day09::lib::part1::part1;
    use crate::year2024::day09::lib::YEAR_2024_DAY_09_SOLUTION;

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_09_SOLUTION.get_parsed_test_inputs(1)),
            "1928"
        );
    }
}
