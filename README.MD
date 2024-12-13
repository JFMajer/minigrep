# Minigrep

Minigrep is a simple search tool implemented in Rust, built by following Chapter 12 of the *The Rust Programming Language* book. It mimics the functionality of the Unix `grep` command, allowing you to search for a phrase in a file with optional case-insensitive searching based on an environment variable.

## Overview

This project implements a basic version of the `grep` utility, which searches for a specified string within a file. Additionally, it can optionally ignore case when performing the search by checking the `IGNORE_CASE` environment variable.

## How It Works

1. The program reads two arguments:
   - The search string.
   - The filename to search within.
2. It checks if the `IGNORE_CASE` environment variable is set. If it's set, the search will be case-insensitive.
3. It reads the file's content and searches for the specified string.
4. If matches are found, they are printed to the console, showing the line number and content.
5. If no matches are found, the program outputs an error message.

## Usage

To run the program, pass two arguments:
- The string you want to search for.
- The file you want to search within.

Example:
```bash
cargo run "search_phrase" "filename.txt"

```bash
export IGNORE_CASE=1
cargo run "search_phrase" "filename.txt"