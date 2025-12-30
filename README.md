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
git clone https://github.com/shimech/ikoy.git
cd ikoy
cargo build --release
```

The binary will be available at `target/release/ikoy`.

## Usage

### Execute a JavaScript File

Run a JavaScript file directly, just like Node.js:

```bash
ikoy script.js
```

Example:

```bash
ikoy javascript/example.js
# Output:
# Hello, This is a JavaScript file!
# 1 + 2 = 3
```

### Evaluate Inline JavaScript

Use the `--print` (or `-p`) flag to evaluate JavaScript code and print the result:

```bash
ikoy -p "1 + 2"
# Output: 3
```

```bash
ikoy -p "'Hello' + ' World!'"
# Output: Hello World!
```

## Command Line Options

```
Usage: ikoy [OPTIONS] [script.js]

Arguments:
  [script.js]  Path to the JavaScript file

Options:
  -p, --print <...>  Evaluate JavaScript code and print result
  -h, --help         Print help
  -V, --version      Print version
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
cargo run -- javascript/example.js
cargo run -- -p "your code here"
```

## Dependencies

- [v8](https://crates.io/crates/v8) - V8 JavaScript engine bindings for Rust
- [clap](https://crates.io/crates/clap) - Command line argument parser

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

This is a learning project for studying JavaScript runtime implementation with Rust and V8.

## Author

Shuntaro Shimizu
