# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`stupidf` is a Rust library with Python bindings for parsing STDF (Standard Test Data Format) files used in semiconductor testing. The library converts binary STDF files into Polars DataFrames for easier analysis.

## Build Commands

- **Build Rust library**: `cargo build`
- **Build release**: `cargo build --release` 
- **Run CLI**: `cargo run -- <stdf_file>`
- **Install CLI**: `cargo install --path .`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`
- **Run tests**: `cargo test`

## CLI Usage

- **Basic parsing**: `cargo run -- <file.stdf>`
- **Verbose mode**: `cargo run -- --verbose <file.stdf>`
- **Show dataframes**: `cargo run -- --df <file.stdf>`
- **Show summary**: `cargo run -- --summarize <file.stdf>`
- **Validate file**: `cargo run -- --validate <file.stdf>`
- **Strict validation**: `cargo run -- --validate --strict <file.stdf>`

## Python Development

- **Build Python bindings**: `maturin develop` (requires `pip install maturin`)
- **Build Python wheel**: `maturin build`

### Python API

- **Parse STDF**: `stupidf.parse_stdf(filename)` - Returns dict with dataframes and metadata
- **Get rows**: `stupidf.get_rows(filename)` - Returns list of device test result dictionaries  
- **Get raw STDF**: `stupidf.get_raw_stdf(filename)` - Returns raw STDF structure as nested dict
- **Validate STDF**: `stupidf.validate_stdf(filename, strict=False)` - Returns ValidationReport object

## Architecture

### Core Components

- **`src/data.rs`**: Main `STDF` struct and `Row` for individual device test results
- **`src/records/`**: STDF record type definitions and parsing logic
- **`src/test_information.rs`**: Test metadata structures (`TestInformation`, `FullTestInformation`)
- **`src/validation.rs`**: STDF file validation and health check functionality
- **`src/data_py.rs`**: Python bindings using PyO3
- **`src/main.rs`**: CLI application with verbose, dataframe, summarize, and validation modes

### Key Data Structures

- **`STDF`**: Top-level structure containing parsed file data
- **`Row`**: Individual device test results with parametric/functional test data
- **`TestInformation`**: Metadata for each test (limits, units, flags)
- **`Records`**: Raw STDF record container with parsing logic

### STDF Record Types

The library implements a subset of STDF record types focused on test data:
- **PTR**: Parametric test records
- **FTR**: Functional test records  
- **TSR**: Test synopsis records
- **PIR/PRR**: Part information/results records
- Additional records in `src/records/records.rs`

### Data Flow

1. Raw STDF file → `Records` (binary parsing)
2. `Records` → `STDF` (structured data with test metadata)
3. `STDF` → Polars DataFrame (for analysis)
4. Python bindings expose DataFrame via `parse_stdf()` function

## Development Notes

- Uses Polars for efficient DataFrame operations
- PyO3 for Python bindings with abi3 compatibility (Python 3.9+)
- STDF files are binary linked-list format requiring sequential parsing
- Not all STDF record types are implemented - add new ones following existing patterns in `src/records/records.rs`
- Set `PYO3_PYTHON` environment variable to avoid unnecessary recompilation