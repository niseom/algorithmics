/**
 * Author: [Your Name]
 * Date: [Date]
 * Description: This code contains two functions that work together to find the nth prime number.
 * The first function, is_prime, takes a positive integer n and returns true if n is prime, and false
 * otherwise. The function uses a simple iterative algorithm to check if n is divisible by any integer
 * between 2 and n/2. The second function, nth_prime, takes a positive integer n and returns the nth
 * prime number. The function uses a loop to iterate over consecutive integers and counts how many
 * prime numbers it has found so far, until it reaches the nth prime. This implementation is simple
 * and efficient for small inputs, but it may not scale well for very large values of n, due to the
 * sequential nature of the algorithm. Additionally, there are more sophisticated algorithms for
 * finding prime numbers that have better asymptotic time complexity, such as the Sieve of Eratosthenes.
 */

// Check if a positive integer n is prime
fn is_prime(n: u64) -> bool {
    // Special case for 1 and smaller numbers, which are not prime
    if n <= 1 {
        return false;
    }
    // Check if n is divisible by any integer between 2 and n/2
    for i in 2..=n/2 {
        if n % i == 0 {
            return false; // n is not prime
        }
    }
    return true; // n is prime
}

// Find the nth prime number
fn nth_prime(n: u64) -> u64 {
    let mut count = 0; // Counter for how many prime numbers we have found so far
    let mut i = 2; // Start checking from 2, the first prime number
    // Loop until we have found the nth prime number
    while count < n {
        if is_prime(i) {
            count += 1; // Increment the counter if we find a prime number
        }
        i += 1; // Move on to the next number
    }
    return i - 1; // Return the last prime number we found
}
