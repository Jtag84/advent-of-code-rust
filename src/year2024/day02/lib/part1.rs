use crate::year2024::day02::lib::parser::ParsedInput;

pub fn part1(reports: ParsedInput) -> String {
    reports.into_iter().filter(is_safe).count().to_string()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let differences: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    (differences.iter().all(|v| v.is_negative()) || differences.iter().all(|v| v.is_positive()))
        && differences.iter().all(|v| {
            let v = v.abs();
            v >= 1 && v <= 3
        })
}

#[cfg(test)]
mod test {
    use crate::year2024::day02::lib::{part1, YEAR_2024_DAY_02_SOLUTION};

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(YEAR_2024_DAY_02_SOLUTION.get_parsed_test_inputs(1)),
            "2"
        );
    }
}
