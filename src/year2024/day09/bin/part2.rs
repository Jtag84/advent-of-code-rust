use crate::DiskSpace::{Free, Id};
use adv_code::year2024::day09::lib::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use itertools::Itertools;

const INPUT_FILE: &str = "input/2024/09/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 09, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 6469636832766);

    Ok(())
}

type Size = usize;

#[derive(Debug, Copy, Clone)]
enum DiskSpace {
    Free(Size),
    Id(usize, Size),
}

fn part2(file_input_path: &str) -> usize {
    let disk_layout = parse_input(&file_input_path);

    let mut compacted_disk: Vec<_> = disk_layout
        .into_iter()
        .enumerate()
        .map(|(index, space)| {
            if index % 2 == 0 {
                Id(index / 2, space)
            } else {
                Free(space)
            }
        })
        .collect();

    let mut processing_id = (compacted_disk.len() - 1) / 2;
    loop {
        let Some((right_index_id, Id(id, id_space_size))) = compacted_disk
            .iter()
            .find_position(|disk_space| match disk_space {
                Free(_) => false,
                Id(id, _) => *id == processing_id,
            })
            .map(|(id, disk_space)| (id, disk_space.clone()))
        else {
            panic!("Wrong disk space")
        };

        if processing_id == 0 {
            break;
        }

        let index_leftmost_free_space = compacted_disk
            .iter()
            .find_position(|disk_space| match disk_space {
                Free(free_space_size) => *free_space_size >= id_space_size,
                Id(_, _) => false,
            })
            .filter(|(index, _)| *index < right_index_id)
            .map(|(id, disk_space)| (id, disk_space.clone()));

        match index_leftmost_free_space {
            None => {}
            Some((index_free, Free(free_space_size))) => {
                if id_space_size < free_space_size {
                    compacted_disk.remove(index_free);
                    compacted_disk.insert(index_free, Free(free_space_size - id_space_size));
                    compacted_disk.insert(index_free, Free(id_space_size));
                    compacted_disk.swap(index_free, right_index_id + 1);
                } else {
                    compacted_disk.swap(index_free, right_index_id);
                }
            }
            _ => unreachable!(),
        }
        processing_id = id - 1;
    }

    let mut checksum = 0;

    let mut index = 0;
    for disk_space in compacted_disk {
        match disk_space {
            Free(free_space) => index += free_space,
            Id(id, size) => {
                checksum += (index..(index + size)).map(|i| i * id).sum::<usize>();
                index += size;
            }
        }
    }

    checksum
}

#[cfg(test)]
mod test {
    use crate::part2;
    use adv_code::year2024::day09::lib::parser::parse_input;

    const TEST_INPUT_FILE: &str = "input/2024/09/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 2858);
    }

    #[test]
    fn test_parser() {
        let parsed = parse_input(TEST_INPUT_FILE);
        println!("Result = {:?}", parsed);
    }

    #[test]
    fn test_vec_replacement() {
        let mut v: Vec<usize> = vec![0, 1, 2, 3, 4];

        let i = *v.get(2).unwrap();

        let p = v.get_mut(i).unwrap();
        *p = 9;
        assert_eq!(v, vec![0, 1, 2, 9, 4]);

        let a = v.iter().find(|x| **x == 4).unwrap();

        assert_eq!(*a, 4);
    }
}
