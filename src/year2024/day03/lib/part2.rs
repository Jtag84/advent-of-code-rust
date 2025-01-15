use crate::year2024::day03::lib::parser::OperationInstruction::Do;
use crate::year2024::day03::lib::parser::OperationInstruction::Dont;
use crate::year2024::day03::lib::parser::OperationInstruction::Mul;
use crate::year2024::day03::lib::parser::ParsedInput;

pub fn part2(operation_instructions: ParsedInput) -> String {
    let mut is_do = true;
    let mut total = 0;
    for operation_instruction in operation_instructions {
        match operation_instruction {
            Mul(l, r) => {
                if is_do {
                    total += l * r
                }
            }
            Do => is_do = true,
            Dont => is_do = false,
        }
    }
    total.to_string()
}

#[cfg(test)]
mod test {
    use crate::year2024::day03::lib::{part2, YEAR_2024_DAY_03_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_03_SOLUTION.get_parsed_test_inputs(2)),
            "48"
        );
    }
}
