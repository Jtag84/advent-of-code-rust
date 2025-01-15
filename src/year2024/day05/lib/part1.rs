use crate::year2024::day05::lib::is_valid;
use crate::year2024::day05::lib::parser::ParsedInput;

pub fn part1((rules, updates): ParsedInput) -> String {
    updates
        .iter()
        .filter(|update_line| is_valid(&rules, update_line))
        .map(|update_line| update_line[update_line.len() / 2])
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day05::lib::{part1, YEAR_2024_DAY_05_SOLUTION};

    #[test]
    fn part1_test() {
        let result = part1(YEAR_2024_DAY_05_SOLUTION.get_parsed_test_inputs(1));
        assert_eq!(result, "143");
    }
}
