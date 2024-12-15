use adv_code::year2024::day02::parser::parse_input;
use adv_code::*;
use anyhow::*;
use code_timing_macros::time_snippet;

const INPUT_FILE: &str = "input/2024/02/inputs.txt";

fn main() -> Result<()> {
    start_day(2024, 02, 1);

    let result = time_snippet!(part1(INPUT_FILE));
    println!("Result = {}", result);

    assert_eq!(result, 591);

    Ok(())
}

fn part1(file_input_path: &str) -> usize {
    let reports = parse_input(&file_input_path);
    reports.into_iter().filter(is_safe).count()
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
    use crate::part1;

    const TEST_INPUT_FILE: &str = "input/2024/02/test_inputs_part1.txt";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT_FILE), 2);
    }
}
