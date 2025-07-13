// Count digits in a number ex 12345 -> 5
pub fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n != 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

// Concatenate two numbers ex 123 and 45 -> 12345
pub fn concat_numbers(a: u64, b: u64) -> u64 {
    let digits_b = count_digits(b);
    a * 10u64.pow(digits_b) + b
}
