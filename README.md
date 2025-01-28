# Advent of Code Solutions in Rust

A high-performance Rust framework for solving Advent of Code puzzles with automated testing, input management, and
performance benchmarking. Currently includes complete solutions for Advent of Code 2024.

## Features

- **Automated Setup**: Shell script to scaffold new day solutions
- **Input Management**: Automatic download of puzzle inputs and test cases
- **Performance Benchmarking**: Built-in system measuring execution time and memory usage
- **Test Framework**: Integrated testing for parsers and solutions
- **Multi-Year Support**: Extensible structure supporting multiple AoC years

## Project Structure

```
.
├── src/
│   ├── yearXXXX/
│   │   └── dayXX/
│   │       └── lib/
│   │           ├── mod.rs
│   │           ├── parser.rs
│   │           ├── part1.rs
│   │           └── part2.rs
├── input/
│   └── XXXX/
│       └── XX/
├── template/
└── cookieSession.txt
```

## Getting Started

1. **Setup**:

```bash
git clone <repository>
cd <repository>
```

2. **Configure Input Download**:
   Create `cookieSession.txt` with your Advent of Code session cookie.

3. **Add New Solution**:

```bash
./addDayFromTemplate.sh <year> <day>
```

4. **Run Solutions**:

```bash
# Usage
cargo run --bin aoc -- -h               

Usage: aoc [OPTIONS]

Options:
  -i, --iterations <ITERATIONS>  [default: 1]
  -y, --years <YEARS>            
  -d, --days <DAYS>              
  -p, --part <PART>              [possible values: part1, part2, input-parser]
  -h, --help                     Print help
  -V, --version                  Print version

# Run all solutions for 2024
cargo run --bin aoc --release -- -y 2024

# Run specific days
cargo run --bin aoc --release -- -y 2024 -d 1,2,3

# Run specific parts
cargo run --bin aoc --release -- -y 2024 -p parser,part1,part2
```

## Performance

The framework includes detailed performance metrics for each solution. For example, the 2024 solutions show:

```
➜  advent-of-code-rust git:(main) ✗ cargo run --bin aoc --release -- -y 2024      
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/aoc -y 2024`
|        Name        |  Avg time  | Max memory |
| 2024 - 01 - parser |  218.75 µs |   80.00 KB |
| 2024 - 01 - part 1 |  113.08 µs |   48.00 KB |
| 2024 - 01 - part 2 |  116.38 µs |   16.00 KB |
| 2024 - 02 - parser |  384.17 µs |   64.00 KB |
| 2024 - 02 - part 1 |  100.29 µs |       0  B |
| 2024 - 02 - part 2 |  401.04 µs |       0  B |
| 2024 - 03 - parser |  140.54 µs |       0  B |
| 2024 - 03 - part 1 |    2.96 µs |       0  B |
| 2024 - 03 - part 2 |    4.96 µs |       0  B |
| 2024 - 04 - parser |  285.54 µs |  256.00 KB |
| 2024 - 04 - part 1 |    2.35 ms |       0  B |
| 2024 - 04 - part 2 |    3.66 ms |       0  B |
| 2024 - 05 - parser |  287.96 µs |   32.00 KB |
| 2024 - 05 - part 1 |  951.62 µs |       0  B |
| 2024 - 05 - part 2 |    5.58 ms |       0  B |
| 2024 - 06 - parser |  172.71 µs |       0  B |
| 2024 - 06 - part 1 |    1.17 ms |  624.00 KB |
| 2024 - 06 - part 2 |  622.10 ms |    7.03 MB |
| 2024 - 07 - parser |  290.25 µs |   96.00 KB |
| 2024 - 07 - part 1 |    2.30 ms |       0  B |
| 2024 - 07 - part 2 |  921.62 ms |   40.81 MB |
| 2024 - 08 - parser |   91.50 µs |       0  B |
| 2024 - 08 - part 1 |   69.33 µs |       0  B |
| 2024 - 08 - part 2 |  219.67 µs |       0  B |
| 2024 - 09 - parser |   57.54 µs |       0  B |
| 2024 - 09 - part 1 |  173.04 µs |       0  B |
| 2024 - 09 - part 2 |  144.50 ms |       0  B |
| 2024 - 10 - parser |   85.54 µs |   16.00 KB |
| 2024 - 10 - part 1 |  361.00 µs |       0  B |
| 2024 - 10 - part 2 |  350.17 µs |       0  B |
| 2024 - 11 - parser |   23.46 µs |       0  B |
| 2024 - 11 - part 1 |  602.67 µs |   32.00 KB |
| 2024 - 11 - part 2 |   22.48 ms |    6.28 MB |
| 2024 - 12 - parser |  204.17 µs |  144.00 KB |
| 2024 - 12 - part 1 |   20.64 ms |    3.17 MB |
| 2024 - 12 - part 2 |   21.72 ms |   64.00 KB |
| 2024 - 13 - parser |  117.96 µs |   16.00 KB |
| 2024 - 13 - part 1 |    4.08 µs |       0  B |
| 2024 - 13 - part 2 |    3.92 µs |       0  B |
| 2024 - 14 - parser |   55.58 µs |       0  B |
| 2024 - 14 - part 1 |   11.58 µs |       0  B |
| 2024 - 14 - part 2 |   12.80 ms |  544.00 KB |
| 2024 - 15 - parser |  391.92 µs |   32.00 KB |
| 2024 - 15 - part 1 |  214.96 µs |       0  B |
| 2024 - 15 - part 2 |    1.25 ms |   16.00 KB |
| 2024 - 16 - parser |  144.17 µs |  144.00 KB |
| 2024 - 16 - part 1 |    4.90 ms |  224.00 KB |
| 2024 - 16 - part 2 |   10.74 ms |    4.84 MB |
| 2024 - 17 - parser |   76.33 µs |       0  B |
| 2024 - 17 - part 1 |   10.92 µs |       0  B |
| 2024 - 17 - part 2 |   84.62 µs |       0  B |
| 2024 - 18 - parser |  124.33 µs |       0  B |
| 2024 - 18 - part 1 |  713.75 µs |       0  B |
| 2024 - 18 - part 2 |    2.07 ms |       0  B |
| 2024 - 19 - parser |   72.04 µs |       0  B |
| 2024 - 19 - part 1 |    1.47 ms |       0  B |
| 2024 - 19 - part 2 |   12.70 ms |       0  B |
| 2024 - 20 - parser |  167.58 µs |   16.00 KB |
| 2024 - 20 - part 1 |    4.50 ms |       0  B |
| 2024 - 20 - part 2 |   49.66 ms |   16.00 KB |
| 2024 - 21 - parser |   45.92 µs |       0  B |
| 2024 - 21 - part 1 |   84.83 µs |   16.00 KB |
| 2024 - 21 - part 2 |    2.89 ms |       0  B |
| 2024 - 22 - parser |   77.88 µs |       0  B |
| 2024 - 22 - part 1 |    1.16 ms |   48.00 KB |
| 2024 - 22 - part 2 |  169.94 ms |   32.36 MB |
| 2024 - 23 - parser |  987.25 µs |  400.00 KB |
| 2024 - 23 - part 1 |   18.97 ms |   16.00 KB |
| 2024 - 23 - part 2 |  105.45 ms |       0  B |
| 2024 - 24 - parser |  102.00 µs |       0  B |
| 2024 - 24 - part 1 |  299.17 µs |       0  B |
| 2024 - 24 - part 2 |  133.88 µs |       0  B |
| 2024 - 25 - parser |  607.04 µs |   64.00 KB |
| 2024 - 25 - part 1 |  248.88 µs |       0  B |
| 2024 - 25 - part 2 |   10.58 µs |       0  B |
|        Total / Max |    2.18  s |   40.81 MB |
```

## Benchmarking Output

The output shows detailed performance metrics in a table format:

- **Name**: Solution identifier (year-day-part)
- **Avg time**: Average execution time (microseconds to milliseconds)
- **Max memory**: Peak memory usage

## Development

1. Use `addDayFromTemplate.sh` to create new solution files
2. Implement your solution in `part1.rs` and `part2.rs`
3. Run tests with `cargo test`
4. Benchmark your solution with the CLI tool

## Testing

Each solution includes automated tests:

```bash
# Run all tests
cargo test

# Run tests for specific day
cargo test year2024::day01
```