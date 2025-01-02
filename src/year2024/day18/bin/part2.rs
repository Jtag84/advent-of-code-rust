use adv_code::lib::grid_utils::XYCoordinates;
use adv_code::year2024::day18::lib::find_shortest_path_after_dropping_n_bytes;
use adv_code::year2024::day18::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/18/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 18, 2);

    let result @ XYCoordinates(x, y) = time_snippet!(part2(INPUT_FILE, 71, 1024));
    println!("Result = {x},{y}");

    assert_eq!(result, XYCoordinates(27, 60));

    Ok(())
}

fn part2(file_input_path: &str, grid_size: usize, num_bytes_to_drop: usize) -> XYCoordinates {
    let bytes = &parse_input(&file_input_path);
    let current = (bytes.len() - num_bytes_to_drop) / 2 + num_bytes_to_drop;
    let index =
        find_first_blocking_byte(bytes, grid_size, num_bytes_to_drop, bytes.len(), current) - 1;
    bytes[index]
}

fn find_first_blocking_byte(
    bytes_list: &Vec<XYCoordinates>,
    grid_size: usize,
    min_num_bytes_to_drop: usize,
    max_num_bytes_to_drop: usize,
    current_num_bytes_to_drop: usize,
) -> usize {
    let path =
        find_shortest_path_after_dropping_n_bytes(bytes_list, grid_size, current_num_bytes_to_drop);
    if path.is_none() {
        if current_num_bytes_to_drop == max_num_bytes_to_drop {
            current_num_bytes_to_drop
        } else {
            let new_current =
                (current_num_bytes_to_drop - min_num_bytes_to_drop) / 2 + min_num_bytes_to_drop + 1;
            find_first_blocking_byte(
                bytes_list,
                grid_size,
                min_num_bytes_to_drop,
                current_num_bytes_to_drop,
                new_current,
            )
        }
    } else {
        if min_num_bytes_to_drop == current_num_bytes_to_drop {
            panic!("not found");
        } else {
            let new_current = (max_num_bytes_to_drop - current_num_bytes_to_drop) / 2
                + current_num_bytes_to_drop
                + 1;
            find_first_blocking_byte(
                bytes_list,
                grid_size,
                current_num_bytes_to_drop,
                max_num_bytes_to_drop,
                new_current,
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::lib::grid_utils::XYCoordinates;
    use adv_code::year2024::day18::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/18/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE, 7, 12), XYCoordinates(6, 1));
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }
}
