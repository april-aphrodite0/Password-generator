# password-generator
a Password generator written in Rust made by me and my friends!

Copyright (C) 2026 april-aphrodite0, vubrixx, ghmaxx1k, distribuited on the GNU/GPLv2 license

# How to install
    sudo make install

# About
current version: 0.1.0 BETA
current stable: none

A command-line password generator that lets you control the
length and character types of each generated password.

# Usage

Run the program and type a command at the `user:` prompt.

Available commands:

    generate         generate a password
    show warranty    show the warranty notice
    show conditions  show the license conditions
    help             show this message
    exit / quit      exit the program

When you run `generate`, you will be asked how long you want
the password to be, and then you will be asked which character
types to include (lowercase, uppercase, numbers, symbols).

# Features

- Configurable password length
- Choice of lowercase, uppercase, numbers, and symbols
- Guaranteed at least one character from each selected type
- Shuffled output so characters are not in a predictable order
- Built-in GPL warranty and license viewer

# Developer notes

- `src/main.rs` handles the command prompt and user input.
- `src/generator.rs` contains the password generation logic.
- `src/pager.rs` displays long text files (warranty, license)
  using the system `less` pager.

AI code: src/pager.rs, from line 25 to line 37 (used AI snippet as reference)

# Changelog

## 0.1.0 BETA
- Initial password generation with configurable length
- Character type selection (lowercase, uppercase, numbers, symbols)
- Guaranteed one character per selected type
- Shuffled output
- Command prompt loop with `help`, `exit`, `quit`
- GPL notice at startup
- Warranty and conditions viewer