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

/// Check if a position is within the bounds of a grid
pub fn inbounds_grid<T>(x: i32, y: i32, grid: &Vec<Vec<T>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

pub fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
