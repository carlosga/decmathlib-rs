# decmathlib-rs

Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.

Rust learning project.

## Badges

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/carlosga/decmathlib-rs/rust.yml)](https://github.com/carlosga/decmathlib-rs/actions)
[![GitHub last commit](https://img.shields.io/github/last-commit/carlosga/decmathlib-rs)](https://github.com/carlosga/decmathlib-rs)
[![Downloads](https://img.shields.io/crates/d/decmathlib-rs)]([https://github.com/carlosga/decmathlib-rs](https://crates.io/crates/decmathlib-rs))

## Acknowledgements

- [Intel Decimal Floating-Point Math Library](https://www.intel.com/content/www/us/en/developer/articles/tool/intel-decimal-floating-point-math-library.html)
- [FPBench](https://fpbench.org/)
- [readme.so](https://readme.so/editor)

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Build Locally

Clone the project

```bash
  git clone https://github.com/carlosga/decmathlib-rs
```

Go to the project directory

```bash
  cd decmathlib-rs
```

Build (Cargo)

```bash
  cargo build
```
## Running Tests

To run tests, run the following command

```bash
  cargo test
```

To include the serde serialization/deserialization tests, run the following command

```bash
  cargo test --features serde
```
