# Benchmark Tools

This is a Rust-based tool for automating benchmarks and generating charts for the various web frameworks in this repository.

## Features

- Automated benchmarking of all web frameworks
- Chart generation from benchmark results
- Support for running benchmarks on individual frameworks
- JSON result export

## Commands

### `create-chart`
Creates a chart from existing benchmark results:

```bash
cargo run -- create-chart -f ben-results.json -o my-chart.png
```

### `create-default`
Creates a default benchmark results file:

```bash
cargo run -- create-default -o ben-results.json
```

### `run-benchmarks`
Automatically runs benchmarks for all frameworks and generates a chart:

```bash
cargo run -- run-benchmarks -o results.json --chart-output chart.png --port 8080
```

### `run-benchmark`
Runs a benchmark for a specific framework:

```bash
cargo run -- run-benchmark -f gin --port 8080
```

## Dependencies

This tool requires:
- Rust (1.92.0 or higher)
- The `just` command runner
- `wrk` for HTTP benchmarking
- All the framework-specific dependencies mentioned in the main README