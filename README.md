# ikoy

A JavaScript runtime written in Rust, powered by the V8 engine.

## Overview

ikoy is a minimal JavaScript runtime that allows you to execute JavaScript code.

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Build from Source

```bash
git clone <repository-url>
cd ikoy
cargo build --release
```

The binary will be available at `target/release/ikoy`.

## Usage

### Basic Usage

Execute JavaScript code using the `--print` (or `-p`) flag:

```bash
ikoy --print "'Hello' + ' World!'"
# Output: Hello World!
```

### Using console.log

```bash
ikoy --print "console.log('Hello', 'World', 123)"
# Output: Hello World 123
```

### Mathematical Operations

```bash
ikoy --print "2 + 2"
# Output: 4
```

## Command Line Options

```
Usage: ikoy --print <PRINT>

Options:
  -p, --print <PRINT>  Evaluate JavaScript and print result
  -h, --help           Print help
  -V, --version        Print version
```

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build
```

### Running in Debug Mode

```bash
cargo run -- --print "your code here"
```

## Dependencies

- [v8](https://crates.io/crates/v8) - V8 JavaScript engine bindings for Rust
- [clap](https://crates.io/crates/clap) - Command line argument parser

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

This is a learning project for studying JavaScript runtime implementation with Rust and V8.

## Author

Shuntaro Shimizu
