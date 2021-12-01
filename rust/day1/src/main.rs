fn main() {
    let lines: Vec<usize> = include_str!("../input_data.txt")
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();
    let ret_a = calculate_day_a(&lines[..]);
    println!("Day a result is {}", ret_a);
}

fn calculate_day_a(data: &[usize]) -> usize {
    let mut count = 0;
    let mut prev = 0;
    for datum in data.iter() {
        if prev != 0 && datum > &prev {
            count += 1;
        }
        prev = *datum;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::calculate_day_a;
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
}
