use crate::year2024::day11::lib::blink_n;
use crate::year2024::day11::lib::parser::ParsedInput;

pub fn part1(stones: ParsedInput) -> String {
    blink_n(stones, 25).to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day11::lib::{part1, YEAR_2024_DAY_11_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_11_SOLUTION.get_parsed_test_inputs(1)),
            "55312"
        );
    }

    #[test]
    fn test_scan() {
        let offsets = vec![3, 2, 1, 4];
        let positions: Vec<i32> = offsets
            .iter()
            .scan(0, |acc, &x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect();
        println!("{:?}", positions);
    }
}
