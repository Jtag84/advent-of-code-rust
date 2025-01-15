use crate::year2024::day03::lib::parser::OperationInstruction::Mul;
use crate::year2024::day03::lib::parser::ParsedInput;

pub fn part1(operations: ParsedInput) -> String {
    operations
        .iter()
        .map(|operation| match operation {
            Mul(left, right) => left * right,
            _ => 0,
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day03::lib::{part1, YEAR_2024_DAY_03_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_03_SOLUTION.get_parsed_test_inputs(1)),
            "161"
        );
    }
}
