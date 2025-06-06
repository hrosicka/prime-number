# Prime Number Checker

A simple Rust command-line application for checking whether a given number is a prime number.

## Features

- Interactive CLI: Continuously prompts the user to enter a number and checks its primality.
- Handles invalid input gracefully.
- Allows users to exit the program at any time by typing `exit`.
- Efficient primality test implementation for 64-bit unsigned integers.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or newer recommended)
- A terminal or command prompt

### Building

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/hrosicka/prime-number.git
cd prime-number
cargo build --release
```

### Running

Run the application with Cargo:

```bash
cargo run --release
```

You will be prompted to enter a number. The program will indicate whether the number is prime. Type `exit` to quit.

## Example Usage

```
Enter a number (or 'exit' to quit): 17

The number 17 is a prime number.

Enter a number (or 'exit' to quit): 20

The number 20 is not a prime number.

Enter a number (or 'exit' to quit): exit
Program terminated.
```

## How It Works

The application uses an efficient algorithm to check whether a number is prime:

- Returns `false` for numbers less than 2.
- Returns `true` for 2.
- Eliminates even numbers greater than 2.
- Checks divisibility up to the square root of the input number, skipping even numbers.

## License

This project is licensed under the [MIT License](LICENSE).
