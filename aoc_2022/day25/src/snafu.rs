use std::fmt::Write;

pub trait Snafu {
    fn convert_to_isize(&self) -> isize;
    fn convert_from_isize(input: isize) -> Self;
    fn to_string(&self) -> String;
}

impl Snafu for Vec<isize> {
    fn convert_to_isize(&self) -> isize {
        let mut ret = 0;
        for value in self.iter() {
            ret *= 5;
            ret += value;
        }
        ret
    }

    fn convert_from_isize(input: isize) -> Self {
        let mut input = input;
        let mut ret = Vec::new();
        let place = find_max_place(input);
        for place in (0..=place).rev() {
            let (min, max) = get_bounds_lower_place(place);
            let mut this_face = 0;
            while input > max {
                this_face += 1;
                input -= 5_isize.pow(place as u32);
            }
            while input < min {
                this_face -= 1;
                input += 5_isize.pow(place as u32);
            }
            ret.push(this_face);
        }
        ret
    }

    fn to_string(&self) -> String {
        let mut ret = String::new();
        for digit in self.iter() {
            match digit {
                0 | 1 | 2 => write!(ret, "{}", digit).unwrap(),
                -1 => ret.push('-'),
                -2 => ret.push('='),
                _ => (),
            }
        }
        ret
    }
}

fn get_bounds_lower_place(place: usize) -> (isize, isize) {
    let mut min = 0;
    let mut max = 0;
    for _ in 0..place {
        min = min * 5 - 2;
        max = max * 5 + 2;
    }
    (min, max)
}

fn find_max_place(input: isize) -> usize {
    let mut place = 0;
    let mut max = 2;
    let mut min = -2;
    while !(min..=max).contains(&input) {
        place += 1;
        max = max * 5 + 2;
        min = min * 5 - 2;
    }
    place
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_to_isize() {
        assert_eq!(vec![1].convert_to_isize(), 1);
        assert_eq!(vec![2].convert_to_isize(), 2);
        assert_eq!(vec![1, -2].convert_to_isize(), 3);
        assert_eq!(vec![1, -1].convert_to_isize(), 4);
        assert_eq!(vec![1, -1].convert_to_isize(), 4);
        assert_eq!(vec![1, 0].convert_to_isize(), 5);
        assert_eq!(vec![1, 1].convert_to_isize(), 6);
        assert_eq!(vec![1, 2].convert_to_isize(), 7);
        assert_eq!(vec![2, -2].convert_to_isize(), 8);
        assert_eq!(vec![2, -1].convert_to_isize(), 9);
        assert_eq!(vec![2, 0].convert_to_isize(), 10);
        assert_eq!(vec![1, -2, 0].convert_to_isize(), 15);
        assert_eq!(vec![1, -1, 0].convert_to_isize(), 20);
        assert_eq!(vec![1, -2, 1, 1, -1, 2].convert_to_isize(), 2022);
        assert_eq!(vec![1, -1, 0, -1, -1, -1, 0].convert_to_isize(), 12345);
        assert_eq!(
            vec![1, 1, 2, 1, -1, 1, 1, 1, 0, -1, 1, -2, 0].convert_to_isize(),
            314159265
        );
    }

    #[test]
    fn test_conversion_from_isize() {
        assert_eq!(vec![1], Vec::convert_from_isize(1));
        assert_eq!(vec![2], Vec::convert_from_isize(2));
        assert_eq!(vec![1, -2], Vec::convert_from_isize(3));
        assert_eq!(vec![1, -1], Vec::convert_from_isize(4));
        assert_eq!(vec![1, -1], Vec::convert_from_isize(4));
        assert_eq!(vec![1, 0], Vec::convert_from_isize(5));
        assert_eq!(vec![1, 1], Vec::convert_from_isize(6));
        assert_eq!(vec![1, 2], Vec::convert_from_isize(7));
        assert_eq!(vec![2, -2], Vec::convert_from_isize(8));
        assert_eq!(vec![2, -1], Vec::convert_from_isize(9));
        assert_eq!(vec![2, 0], Vec::convert_from_isize(10));
        assert_eq!(vec![1, -2, 0], Vec::convert_from_isize(15));
        assert_eq!(vec![1, -1, 0], Vec::convert_from_isize(20));
        assert_eq!(vec![1, -2, 1, 1, -1, 2], Vec::convert_from_isize(2022));
        assert_eq!(
            vec![1, -1, 0, -1, -1, -1, 0],
            Vec::convert_from_isize(12345)
        );
        assert_eq!(
            vec![1, 1, 2, 1, -1, 1, 1, 1, 0, -1, 1, -2, 0],
            Vec::convert_from_isize(314159265)
        );
    }
}
