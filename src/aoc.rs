use adv_code::Part::{InputParser, Part1, Part2};
use adv_code::{BenchmarkMetrics, Day, Part, Year, SOLUTIONS};
use clap::Parser;
use itertools::Itertools;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    iterations: usize,

    #[arg(short, long)]
    years: Vec<Year>,

    #[arg(short, long)]
    days: Vec<Day>,

    #[arg(short, long)]
    part: Vec<Part>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let benchmarks = SOLUTIONS
        .lock()
        .unwrap()
        .iter()
        .filter(|((year, _), _)| args.years.is_empty() || args.years.contains(year))
        .filter(|((_, day), _)| args.days.is_empty() || args.days.contains(day))
        .sorted_by_key(|(year_day, _)| *year_day)
        .flat_map(|(_, solution)| {
            let mut solutions_parts = vec![];

            if args.part.is_empty() || args.part.contains(&InputParser) {
                solutions_parts.push(solution.benchmark_parser(args.iterations));
            }
            if args.part.is_empty() || args.part.contains(&Part1) {
                solutions_parts.push(solution.solve_part1(args.iterations));
            }
            if args.part.is_empty() || args.part.contains(&Part2) {
                solutions_parts.push(solution.solve_part2(args.iterations));
            }

            solutions_parts
        })
        .collect_vec();

    let total_avg_time = benchmarks
        .iter()
        .map(|benchmark| benchmark.avg_duration())
        .sum::<Duration>();

    let max_memory = benchmarks
        .iter()
        .map(|benchmark| benchmark.max_memory())
        .max()
        .unwrap();

    println!("{}", BenchmarkMetrics::header());
    benchmarks
        .iter()
        .for_each(|benchmark| println!("{}", benchmark.row_string()));
    println!("{}", BenchmarkMetrics::footer(total_avg_time, max_memory));
    Ok(())
}
