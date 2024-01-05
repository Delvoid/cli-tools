# echor: A Rust Implementation of Echo

## Introduction

`echor` is a simple Rust-based command-line tool that mimics the functionality of the classic Unix `echo` command. This project serves as a practical application for learning Rust, focusing on basic concepts like STDOUT and STDERR, the subtleties of Rust's `print!` macro, and fundamental file operations.

## Key Features

- **Text Output:** Accepts and concatenates multiple text arguments, demonstrating basic string manipulation in Rust.
- **Newline Control:** Explore the difference in output formatting in Rust by choosing to include or exclude the newline character.
- **Error Handling:** Highlights Rust's error handling in CLI applications, particularly when dealing with incorrect or missing arguments.
- **File Reading:** Showcases simple file reading operations, offering insights into Rust's approach to file I/O.
- **Testing:** Includes a set of tests using `assert_cmd`, providing a hands-on experience with Rust's testing framework.

## How to Use

- To display text: `echor "Your text here"`
- To display text without a newline: `echor -n "Your text here"`

## Learning Aspects

- Command-line argument parsing with the `clap` library.
- Differentiating `print!` and `println!` macros in Rust.
- Understanding standard output streams in Rust, like STDOUT and STDERR.
- Developing tests in Rust using `assert_cmd`.

---

_A Rust Learning Project_
