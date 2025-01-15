use crate::lib::grid_utils::XYCoordinates;
use crate::year2024::day18::lib::find_shortest_path_after_dropping_n_bytes;
use crate::year2024::day18::lib::parser::ParsedInput;

pub fn part2(bytes: ParsedInput, grid_size: usize, num_bytes_to_drop: usize) -> XYCoordinates {
    let current = (bytes.len() - num_bytes_to_drop) / 2 + num_bytes_to_drop;
    let index =
        find_first_blocking_byte(&bytes, grid_size, num_bytes_to_drop, bytes.len(), current) - 1;
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
    use crate::lib::grid_utils::XYCoordinates;
    use crate::year2024::day18::lib::{part2, YEAR_2024_DAY_18_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_18_SOLUTION.get_parsed_test_inputs(2), 7, 12),
            XYCoordinates(6, 1)
        );
    }
}
