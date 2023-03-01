/**
 * Author: NISEOM
 * Date: 2023
 * Description: This code implements a recursive function to compute the nth Fibonacci number.
 * The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones.
 * The function uses pattern matching to handle the base cases (n = 0, n = 1) and a recursive call
 * to compute the nth Fibonacci number for larger values of n. This implementation is simple and
 * easy to understand, but it has exponential time complexity and may not be efficient for large inputs.
 */

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => return 0, // Base case: fib(0) = 0
        1 => return 1, // Base case: fib(1) = 1
        _ => return fibonacci(n-1) + fibonacci(n-2) // Recursive case: fib(n) = fib(n-1) + fib(n-2)
    }
}

