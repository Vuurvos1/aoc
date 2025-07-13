# AOC 2024

## Fetching Inputs

Create a `.env` file based on the `.env.example` file.
You can get the session cookie from the browser dev tools.

```bash
# fetch input for a specific day (defaults to current year)
cargo run --bin fetch_input 1

# fetch input for a specific day and year
cargo run --bin fetch_input 1 2023
```

## Running

```bash
# single day
cargo run --bin main 1

# all days
cargo run --bin main all
```

## Benchmarking

```bash
# all days
cargo bench

# single days
cargo bench d01

# utility benchmarks
cargo bench utilities
```

> [!NOTE]
> Note that I didn't solve all days using code, so not every day/part has a benchmark.
