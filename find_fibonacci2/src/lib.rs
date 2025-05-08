pub fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 0 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
