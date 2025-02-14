
# Word Counter Project

## Overview
This project reads a file and counts how many times a specific word appears in it. To achieve this, we need to:

1. Read the contents of a file.
2. Get a word from the user to search for in the file.

## Getting User Input
There are two ways to receive the word from the user:
- Using `stdin().read_line()` to take input from the terminal.
- Passing the word as a command-line argument.

## Handling Process Exit
If the program encounters an issue, it should exit properly. In Rust, we can use:

```rust
std::process::exit(1);
```

- `std::process::exit(code: i32)` immediately stops the program and returns a status code to the operating system.
- `exit(1);` signals an error, while `exit(0);` means successful execution.

## Counting Word Occurrences
Initially, I tried using `.find(query)`, but it returned unexpected results. This is because:

- `.find(query)` returns the **byte index** of the first occurrence of `query`, not the number of times it appears.
- To properly count occurrences, we need to split the file’s content into words.

## Splitting Words Correctly
At first, I used `.split(" ")`, but it gave incorrect results. The issue was:

- `.split(" ")` only breaks text at spaces.
- Real text also contains **newlines, tabs, and extra spaces**, which `.split(" ")` doesn’t handle well.
- Instead, `.split_whitespace()` is a better option because it properly separates words regardless of spacing.

## Future Improvements
- Make the search **case-insensitive** so that uppercase and lowercase letters don’t affect results.

## Useful Rust Conversions
- Convert a `String` to `&str` using `.as_str()`.
- Convert a `&str` to `String` using `.to_string()`.

This project helped me understand how text processing works in Rust and the importance of using the right string methods. The next step is improving the accuracy of word matching by ignoring case differences.


