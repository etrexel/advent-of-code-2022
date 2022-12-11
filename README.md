# 🎄 Advent of Code 2022 🎄

[![Rust](https://github.com/etrexel/advent-of-code-2022/actions/workflows/rust.yml/badge.svg)](https://github.com/etrexel/advent-of-code-2022/actions/workflows/rust.yml)
[![docs](https://img.shields.io/badge/rustdoc-latest-orange)](https://blog.trexel.io/advent-of-code-2022/aoc/)
[![codecov](https://codecov.io/gh/etrexel/advent-of-code-2022/branch/main/graph/badge.svg?token=E8J0GE2R24)](https://codecov.io/gh/etrexel/advent-of-code-2022)
[![license](https://img.shields.io/badge/license-Apache--2.0-blue)](https://github.com/etrexel/advent-of-code-2022/blob/main/LICENSE)

**Solutions to Advent of Code 2022**

# Getting started
## Prerequisites
Make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed.

## Build
Clone the repo and build the project:
```shell
git clone https://github.com/etrexel/advent-of-code-2022.git
cd advent-of-code-2022
make build
```

You should now be able to run the binary:
```shell
./target/debug/aoc --help
Solver for Advent of Code 2022

Usage: aoc [OPTIONS]

Options:
  -d, --day <DAY>    Which day's puzzle to solve [default: 1]
  -p, --part <PART>  Which part of the day's puzzle to solve [default: 1]
  -f, --file <FILE>  Path to input file
  -h, --help         Print help information
  -V, --version      Print version information
```

Sample inputs for each day are organized under the `input` directory. These files will automatically be used if you
don't provide an input file.
