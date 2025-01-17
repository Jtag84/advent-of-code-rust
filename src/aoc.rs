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

    println!("{}", BenchmarkMetrics::header());

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
                print_and_insert(
                    &mut solutions_parts,
                    solution.benchmark_parser(args.iterations),
                );
            }
            if args.part.is_empty() || args.part.contains(&Part1) {
                print_and_insert(&mut solutions_parts, solution.solve_part1(args.iterations));
            }
            if args.part.is_empty() || args.part.contains(&Part2) {
                print_and_insert(&mut solutions_parts, solution.solve_part2(args.iterations));
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
        .unwrap_or(0);

    println!("{}", BenchmarkMetrics::footer(total_avg_time, max_memory));
    Ok(())
}

fn print_and_insert(solutions_parts: &mut Vec<BenchmarkMetrics>, metrics: BenchmarkMetrics) {
    println!("{}", metrics.row_string());
    solutions_parts.push(metrics);
}
