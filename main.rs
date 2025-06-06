use std::io::{self, Write};

/// Entry point of the program.
/// Repeatedly prompts the user to enter a number and checks if it is a prime number.
/// The loop can be terminated by typing 'exit'.
fn main() {
    loop {
        // Prompt the user for input
        print!("Enter a number (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        // Read and trim user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        // Exit condition
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Program terminated.");
            break;
        }

        // Attempt to parse the input as a u64 number
        let num: u64 = match trimmed.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("\nThe entered value is not a valid number.");
                continue;
            }
        };

        // Check if the number is prime and display the result
        if is_prime(num) {
            println!("\nThe number {} is a prime number.\n", num);
        } else {
            println!("\nThe number {} is not a prime number.\n", num);
        }
    }
}

/// Determines if a given number is a prime number.
///
/// # Arguments
///
/// * `n` - The number to be checked.
///
/// # Returns
///
/// * `true` if the number is prime.
/// * `false` otherwise.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    // Check divisibility up to the square root of n, skipping even numbers
    let max = (n as f64).sqrt() as u64 + 1;
    for i in (3..max).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}