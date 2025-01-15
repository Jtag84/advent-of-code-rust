use crate::year2024::day02::lib::parser::ParsedInput;

pub fn part2(reports: ParsedInput) -> String {
    reports
        .into_iter()
        .filter(is_safe_up_to_one_wrong)
        .count()
        .to_string()
}

fn is_safe_up_to_one_wrong(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report_with_one_removed = report.clone();
        report_with_one_removed.remove(i);

        if is_safe(&report_with_one_removed) {
            return true;
        }
    }

    false
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
    use crate::year2024::day02::lib::{part2, YEAR_2024_DAY_02_SOLUTION};

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(YEAR_2024_DAY_02_SOLUTION.get_parsed_test_inputs(2)),
            "4"
        );
    }
}
