# catr: A Rust Implementation of Cat

## Introduction

`catr` is a Rust-based command-line tool that replicates some functionalities of the classic Unix `cat` command. Designed as a learning project, it currently implements the `-n` and `-b` flags, allowing users to number lines and number non-blank lines, respectively. This project offers a hands-on experience with Rust, focusing on command-line argument parsing, file I/O, and conditional operations based on flags.

## Key Features

- **Line Numbering:** The `-n` flag numbers each line of the output.
- **Non-Blank Line Numbering:** The `-b` flag numbers only non-blank lines.
- **File Reading:** Demonstrates file reading capabilities in Rust.
- **Command-Line Argument Parsing:** Utilizes the `clap` library for parsing command-line arguments.
- **Error Handling:** Robust error handling for file reading operations and argument parsing.

## How to Use

- To display the contents of a file: `catr [file_name]`
- To number each line of the output: `catr -n [file_name]`
- To number only non-blank lines: `catr -b [file_name]`

## Learning Aspects

- File reading and output formatting in Rust.
- Parsing command-line arguments using the `clap` library.
- Implementing and understanding command-line flags and their effects.
- Handling errors in file operations and command-line inputs.

## TODO

- Implement other `cat` flags such as `-E` (display ends of lines), `-s` (suppress repeated empty lines), etc.

---

_A Rust Learning Project_
