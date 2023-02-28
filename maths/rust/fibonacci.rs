fn fibonacci(n: u32) -> u32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => return fibonacci(n-1) + fibonacci(n-2)
    }
}
