pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    a * (b / greatest_common_divisor(a, b))
}
