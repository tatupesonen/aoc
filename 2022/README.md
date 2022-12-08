# 2022
Language: [Rust](https://www.rust-lang.org/)

## Usage
All commands below should be ran in this folder (2022).

### Test
```console
cargo test
```

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

### Show all commands
```console
cargo run --release -- --help

Usage: aoc2022 [OPTIONS]

Options:
  -d, --day <DAY>  Day to run
  -t, --test       Flag to enable test input
  -h, --help       Print help information
  -V, --version    Print version information
```

