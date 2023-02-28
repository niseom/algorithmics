fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn nth_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut i = 2;
    while count < n {
        if is_prime(i) {
            count += 1
        }
        i += 1;
    }
    return i - 1;
}