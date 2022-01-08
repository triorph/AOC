fn main() {
    let lines: Vec<usize> = include_str!("../input_data.txt")
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();
    let ret_a = calculate_day_a(&lines[..]);
    println!("Day a result is {}", ret_a);
    let ret_b = calculate_day_b(&lines[..]);
    println!("Day b result is {}", ret_b);
}

fn calculate_num_increases_any_gap(data: &[usize], gap: usize) -> usize {
    // Calculate how many increases there are in the data, from points a to points a + gap.
    let mut count = 0;
    for (datum, prev) in Iterator::zip(data[gap..].iter(), data[..data.len() - gap].iter()) {
        if datum > prev {
            count += 1;
        }
    }
    count
}

fn calculate_day_a(data: &[usize]) -> usize {
    calculate_num_increases_any_gap(data, 1)
}

fn calculate_day_b(data: &[usize]) -> usize {
    // Although we are supposed to be comparing sums of a sliding window
    // in actuality sum (a b c) compared to sum (b c d) just means you have
    // to compare d vs a, so the main issue is keeping track of the last 3 data points
    calculate_num_increases_any_gap(data, 3)
}

#[cfg(test)]
mod test {
    use crate::calculate_day_a;
    use crate::calculate_day_b;
    #[test]
    fn test_day_a() {
        let lines: Vec<usize> = include_str!("../test_data.txt")
            .lines()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect();
        let ret = calculate_day_a(&lines[..]);
        assert_eq!(ret, 7);
    }

    #[test]
    fn test_day_b() {
        let lines: Vec<usize> = include_str!("../test_data.txt")
            .lines()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect();
        let ret = calculate_day_b(&lines[..]);
        assert_eq!(ret, 5);
    }
}
