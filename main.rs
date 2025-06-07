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

#[cfg(test)] // This directive indicates that the following code will only be compiled when running tests
mod tests {
    use super::*; // Imports all items from the parent module (in this case, the is_prime function)

    #[test]
    fn test_is_prime_small_numbers() {
        // Test basic cases
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);  // 2 is the only even prime number
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false); // 4 = 2*2
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false); // 6 = 2*3
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false); // 8 = 2*4
        assert_eq!(is_prime(9), false); // 9 = 3*3
        assert_eq!(is_prime(10), false); // 10 = 2*5
    }

    #[test]
    fn test_is_prime_some_primes() {
        // Test known prime numbers
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);
        assert_eq!(is_prime(29), true);
        assert_eq!(is_prime(31), true);
        assert_eq!(is_prime(97), true);
        assert_eq!(is_prime(199), true);
        assert_eq!(is_prime(997), true);
    }

    #[test]
    fn test_is_prime_some_composites() {
        // Test some composite numbers
        assert_eq!(is_prime(25), false); // 5*5
        assert_eq!(is_prime(33), false); // 3*11
        assert_eq!(is_prime(49), false); // 7*7
        assert_eq!(is_prime(51), false); // 3*17
        assert_eq!(is_prime(81), false); // 9*9
        assert_eq!(is_prime(100), false); // 10*10
        assert_eq!(is_prime(121), false); // 11*11
        assert_eq!(is_prime(169), false); // 13*13
        assert_eq!(is_prime(200), false);
        assert_eq!(is_prime(1000), false);
    }

    #[test]
    fn test_is_prime_large_prime() {
        // Test a larger prime number (it's important to test the function's efficiency)
        assert_eq!(is_prime(1000003), true); // Example of a large prime
        assert_eq!(is_prime(999983), true);  // Another large prime
    }

    #[test]
    fn test_is_prime_large_composite() {
        // Test a larger composite number
        assert_eq!(is_prime(1000000), false); // Even
        assert_eq!(is_prime(1000001), false); // Divisible by 101, 9901
        assert_eq!(is_prime(1000000007 * 2), false); // Large even
        assert_eq!(is_prime(1000000007), true); // Large prime, for comparison
    }

    #[test]
    fn test_is_prime_square_of_prime() {
        // Special test for a prime's square, where a loop range error might occur
        assert_eq!(is_prime(4), false);   // 2*2
        assert_eq!(is_prime(9), false);   // 3*3
        assert_eq!(is_prime(25), false);  // 5*5
        assert_eq!(is_prime(121), false); // 11*11
        assert_eq!(is_prime(169), false); // 13*13
    }
}