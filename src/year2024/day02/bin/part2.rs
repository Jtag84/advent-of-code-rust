use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use adv_code::year2024::day02::parser::parse_input;

const INPUT_FILE: &str = "input/2024/02/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 02, 2);

    let result = time_snippet!(part2(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 621);

    Ok(())
}

fn part2(file_input_path: &str) -> usize {
    let reports = parse_input(&file_input_path);
    reports.into_iter().filter(is_safe_up_to_one_wrong).count()
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
    let differences: Vec<i32> = report.iter().zip(report.iter().skip(1))
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
    use crate::part2;

    const TEST_INPUT_FILE: &str = "input/2024/02/test_inputs_part2.txt";

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT_FILE), 4);
    }
}