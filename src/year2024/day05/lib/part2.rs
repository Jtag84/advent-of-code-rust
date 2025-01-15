use crate::year2024::day05::lib::is_valid;
use crate::year2024::day05::lib::parser::ParsedInput;

pub fn part2((rules, updates): ParsedInput) -> String {
    let mut updates: Vec<_> = updates
        .into_iter()
        .filter(|update_line| !is_valid(&rules, update_line))
        .collect();

    for update_line in updates.iter_mut() {
        let mut index = 0;
        'update_line_loop: while index < update_line.len() {
            let current_update = *update_line
                .get(index)
                .expect(&format!("error {index} in {update_line:?}"));
            for (next_index_offset, next_update) in update_line[(index + 1)..].iter().enumerate() {
                if rules.contains(&(*next_update, current_update)) {
                    update_line.swap(index, index + 1 + next_index_offset);
                    continue 'update_line_loop;
                }
            }
            index += 1;
        }
    }

    updates
        .into_iter()
        .map(|update_line| *update_line.get(update_line.len() / 2).expect("error"))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day05::lib::{part2, YEAR_2024_DAY_05_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_05_SOLUTION.get_parsed_test_inputs(2)),
            "123"
        );
    }
}
