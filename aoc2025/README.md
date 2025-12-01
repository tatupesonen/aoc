# 2024
AoC 2024 in [Rust](https://www.rust-lang.org/)

## Usage
All commands below should be ran in this folder (aoc2024).

## Tests
To run all tests:
```console
cargo test
```

## Running days
### Run all days
```console
cargo run --release
```

### Run a specific day
```console
cargo run --release -- --day <n>
```

### Run days with test input
```console
cargo run --release -- --test # Runs all with test input
```

## Miscellanous
### Benchmark
```console
cargo bench
```

### Show all commands
```console
cargo run --release -- --help

Usage: aoc2024 [OPTIONS]

Options:
  -d, --day <DAY>  Day to run
  -t, --test       Flag to enable test input
  -h, --help       Print help information
  -V, --version    Print version information
```

