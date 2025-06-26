# KeyValues3

[![Crates.io](https://img.shields.io/crates/v/keyvalues3.svg)](https://crates.io/crates/keyvalues3)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Repository](https://img.shields.io/badge/repository-GitHub-blue.svg)](https://github.com/TheCursedApple/KeyValues3)
[![Workflow Status](https://img.shields.io/github/actions/workflow/status/TheCursedApple/KeyValues3/rust.yml)](https://github.com/TheCursedApple/KeyValues3/actions)

A Rust library and CLI tool for parsing and manipulating Valve's KeyValues3 (KV3) format. KV3 is a data serialization format used in games like Dota 2, CS2, and Deadlock. This tool allows you to parse KV3 files, format them with proper indentation, and convert them to JSON.

## Features

- Parse KV3 files into a Rust data structure
- Format KV3 files with standard indentation and line breaks
- Convert KV3 to JSON (with the "json" feature)
- Command-line interface for easy usage

## Installation

### CLI Tool

To use the CLI tool, download the pre-built binary for your operating system from the [GitHub releases page](https://github.com/TheCursedApple/KeyValues3/releases). Extract the binary and place it in a directory included in your system's PATH.

### Library

To use `keyvalues3` as a library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
keyvalues3 = "1.0.0"
```

For JSON conversion support, enable the "json" feature:

```toml
[dependencies]
keyvalues3 = { version = "1.0.0", features = ["json"] }
```

## Using the CLI Tool

The CLI tool, named `kv3`, provides two primary commands: `format` and `convert`. Use the `--input` option to specify the KV3 file to process.

### Format

The `format` command reformats a KV3 file with proper indentation and line breaks.

**Usage:**

```
kv3-cli --input <input_file> format [--output <output_file>]
```

- `--input`: Path to the input KV3 file (required).
- `--output`: Path to save the formatted KV3 (optional; defaults to stdout).

### Convert

The `convert` command converts a KV3 file to JSON.

**Usage:**

```
kv3-cli --input <input_file> convert [--output <output_file>]
```

- `--input`: Path to the input KV3 file (required).
- `--output`: Path to save the JSON output (optional; defaults to stdout).

### Examples

- Format a KV3 file and print to the console:

  ```
  kv3-cli --input example.kv3 format
  ```

- Format a KV3 file and save to a new file:

  ```
  kv3-cli --input example.kv3 format --output formatted.kv3
  ```

- Convert a KV3 file to JSON and print to the console:

  ```
  kv3-cli --input example.kv3 convert
  ```

- Convert a KV3 file to JSON and save to a new file:

  ```
  kv3-cli --input example.kv3 convert --output example.json
  ```

## Using the Library

The `keyvalues3` crate provides functions to parse, format, and convert KV3 data programmatically in Rust.

### Parsing KV3

Use `kv3::from_str` to parse a KV3 string into a `Value` enum:

```rust
use kv3::{from_str, Value};

let kv3_str = r#"
    <!-- kv3 encoding:text:version{211C2728-7EE6-4494-BDC9-65496C10E5FD} format:generic:version{6E663C66-2F27-4609-90A4-0E0A731B0A3D} -->
    {
        key = "value"
    }
"#;

let value: Value = from_str(kv3_str).expect("Failed to parse KV3");
```

### Formatting KV3

Use `kv3::to_string` to convert a `Value` back into a formatted KV3 string:

```rust
use kv3::to_string;

let formatted_kv3 = to_string(&value);
println!("{}", formatted_kv3);
```

### Converting to JSON

With the "json" feature enabled, use `kv3::to_json` to convert a `Value` to a `serde_json::Value`:

```rust
use kv3::to_json;
use serde_json::Value as JsonValue;

let json_value: JsonValue = to_json(&value);
println!("{}", json_value);
```

Ensure the "json" feature is enabled in your `Cargo.toml` as shown in the installation section.

## Building from Source

To build the CLI tool from source, ensure you have Rust 1.87 or later installed. Clone the repository and run:

```
cargo build --release --features binary,json
```

The binary will be located at `target/release/kv3-cli`. Copy it to a directory in your PATH for easy access.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on the [GitHub repository](https://github.com/TheCursedApple/KeyValues3).
