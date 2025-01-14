use clap::ValueEnum;
use downcast_rs::{impl_downcast, DowncastSync};
use itertools::Itertools;
use memory_stats::memory_stats;
use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub mod year2024;

pub mod lib {
    pub mod grid_utils;
    pub mod parser_commons;
}

pub fn start_day(year: i32, day: i32, part: i32) {
    println!("Advent of Code {year} - Day {:0>2}", day);
    println!("=== Part {part} ===");
}

pub type Year = usize;
pub type Day = usize;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Part {
    Part1,
    Part2,
    InputParser,
}

impl_downcast!(sync ParsedInput);

// Trait to allow type erasure
pub trait ParsedInput: DowncastSync {}
impl<T: Any + Send + Sync + Clone> ParsedInput for T {}

// Trait for solutions to implement
pub trait Solution: Send + Sync {
    fn get_inputs_path(&self) -> String;
    fn get_test_inputs_path(&self, part_number: usize) -> String;
    fn benchmark_parser(&self, iterations: usize) -> BenchmarkMetrics;
    fn solve_part1(&self, iterations: usize) -> BenchmarkMetrics;
    fn solve_part2(&self, iterations: usize) -> BenchmarkMetrics;
    fn solve_part(&self, part_number: usize, iterations: usize) -> BenchmarkMetrics;
}

// Generic solution implementation
pub struct DaySolution<P: ParsedInput + 'static + Clone + Debug> {
    year_day: (Year, Day),
    parser: fn(&str) -> P,
    part1: fn(P) -> String,
    expected_part1: &'static str,
    part2: fn(P) -> String,
    expected_part2: &'static str,
}

impl<P: ParsedInput + 'static + Clone + Debug> DaySolution<P> {
    #[cfg(test)]
    fn get_parsed_test_inputs(&self, part_number: usize) -> P {
        let input_file: String = self.get_test_inputs_path(part_number);
        (self.parser)(&*input_file)
    }
}

impl<P: ParsedInput + 'static + Clone + Debug> Solution for DaySolution<P> {
    fn get_inputs_path(&self) -> String {
        format!("input/{}/{}/inputs.txt", self.year_day.0, self.year_day.1)
    }

    fn get_test_inputs_path(&self, part_number: usize) -> String {
        format!(
            "input/{}/{}/test_inputs_part{}.txt",
            self.year_day.0, self.year_day.1, part_number
        )
    }

    fn benchmark_parser(&self, iterations: usize) -> BenchmarkMetrics {
        let input_file: String = self.get_inputs_path();

        let function_metrics = (0..iterations)
            .map(|_| {
                let start = Instant::now();
                let initial_memory = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

                let parsed_input = (self.parser)(&*input_file);

                let duration = start.elapsed();
                let final_memory = memory_stats().map(|m| m.physical_mem).unwrap_or(0);
                let memory_used = final_memory.saturating_sub(initial_memory);
                (
                    duration,
                    memory_used,
                    String::from(format!("{parsed_input:?}")),
                )
            })
            .collect_vec();

        BenchmarkMetrics {
            name: format!("{} - {:0>2} - parser", self.year_day.0, self.year_day.1),
            metrics: function_metrics,
        }
    }

    fn solve_part1(&self, iterations: usize) -> BenchmarkMetrics {
        self.solve_part(1, iterations)
    }

    fn solve_part2(&self, iterations: usize) -> BenchmarkMetrics {
        self.solve_part(2, iterations)
    }

    fn solve_part(&self, part_number: usize, iterations: usize) -> BenchmarkMetrics {
        let (part_function, expected_result) = match part_number {
            1 => (self.part1, self.expected_part1),
            2 => (self.part2, self.expected_part2),
            _ => panic!("Invalid part_number {}", part_number),
        };
        let input_file: String = self.get_inputs_path();

        let parsed_input = (self.parser)(&*input_file);
        let function_metrics = (0..iterations).map(|i| {
            let parsed_input_cloned = parsed_input.clone();
            let start = Instant::now();
            let initial_memory = memory_stats().map(|m| m.physical_mem).unwrap_or(0);

            let result = part_function(parsed_input_cloned);

            let duration = start.elapsed();
            let final_memory = memory_stats().map(|m| m.physical_mem).unwrap_or(0);
            let memory_used = final_memory.saturating_sub(initial_memory);
            assert_eq!(result, expected_result, "Validating result for {:?} part {part_number} iteration {i} see DaySolution definition at src/year{}/day{}/lib/mod.rs", self.year_day, self.year_day.0, self.year_day.1);
            (duration, memory_used, result)
        }).collect_vec();

        BenchmarkMetrics {
            name: format!(
                "{} - {:0>2} - part {}",
                self.year_day.0, self.year_day.1, part_number
            ),
            metrics: function_metrics,
        }
    }
}

pub static SOLUTIONS: Lazy<Mutex<HashMap<(Year, Day), Box<dyn Solution>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[macro_export]
macro_rules! aoc_solver {
    ($day_solution:expr) => {
        #[ctor::ctor]
        fn register_solution() {
            SOLUTIONS
                .lock()
                .unwrap()
                .insert($day_solution.year_day, Box::new($day_solution));
        }
    };
}

type MaxMemoryBytes = usize;
type StringSolution = String;
type FunctionMetrics = (Duration, MaxMemoryBytes, StringSolution);

pub struct BenchmarkMetrics {
    pub name: String,
    pub metrics: Vec<FunctionMetrics>,
}

impl BenchmarkMetrics {
    pub fn number_of_iterations(&self) -> usize {
        self.metrics.len()
    }

    pub fn min_duration(&self) -> Duration {
        self.metrics
            .iter()
            .map(|(duration, _, _)| duration)
            .min()
            .unwrap()
            .clone()
    }

    pub fn max_duration(&self) -> Duration {
        self.metrics
            .iter()
            .map(|(duration, _, _)| duration)
            .max()
            .unwrap()
            .clone()
    }

    pub fn max_memory(&self) -> MaxMemoryBytes {
        self.metrics
            .iter()
            .map(|(_, memory, _)| memory)
            .max()
            .unwrap()
            .clone()
    }

    pub fn avg_duration(&self) -> Duration {
        let nanos: u64 = self
            .metrics
            .iter()
            .map(|(duration, _, _)| duration.as_nanos())
            .sum::<u128>() as u64
            / self.number_of_iterations() as u64;

        Duration::from_nanos(nanos)
    }

    pub fn footer(total_time: Duration, max_memory_bytes: MaxMemoryBytes) -> String {
        format!(
            "| {:>18} | {:>10} | {:>10} |",
            "Total / Max",
            format_duration(total_time),
            format_bytes(max_memory_bytes)
        )
    }
    pub fn header() -> String {
        format!(
            "| {:^18} | {:^10} | {:^10} |",
            "Name", "Avg time", "Max memory"
        )
    }

    pub fn row_string(&self) -> String {
        format!(
            "| {:>18} | {:>10} | {:>10} |",
            self.name,
            format_duration(self.avg_duration()),
            format_bytes(self.max_memory())
        )
    }
}

pub fn format_duration(duration: Duration) -> String {
    if duration.as_nanos() < 1_000 {
        format!("{} ns", duration.as_nanos())
    } else if duration.as_nanos() < 1_000_000 {
        format!("{:.2} µs", duration.as_nanos() as f64 / 1_000.0)
    } else if duration.as_nanos() < 1_000_000_000 {
        format!("{:.2} ms", duration.as_micros() as f64 / 1_000.0)
    } else {
        format!("{:.2}  s", duration.as_secs_f64())
    }
}

pub fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;

    if bytes < KB {
        format!("{}  B", bytes)
    } else if bytes < MB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    }
}
