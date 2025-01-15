use crate::year2024::day09::lib::parser::ParsedInput;
use crate::year2024::day09::lib::part2::DiskSpace::{Free, Id};
use itertools::Itertools;

type Size = usize;

#[derive(Debug, Copy, Clone)]
enum DiskSpace {
    Free(Size),
    Id(usize, Size),
}

pub fn part2(disk_layout: ParsedInput) -> String {
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

    checksum.to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day09::lib::{part2, YEAR_2024_DAY_09_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_09_SOLUTION.get_parsed_test_inputs(2)),
            "2858"
        );
    }

    #[test]
    fn test_vec_replacement() {
        let mut v: Vec<usize> = vec![0, 1, 2, 3, 4];

        let i = *v.get(3).unwrap();

        let p = v.get_mut(i).unwrap();
        *p = 9;
        assert_eq!(v, vec![0, 1, 2, 9, 4]);

        let a = v.iter().find(|x| **x == 4).unwrap();

        assert_eq!(*a, 4);
    }
}
