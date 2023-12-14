pub trait Transposable<T> {
    fn transpose(&self) -> Vec<Vec<T>>;
}

impl<T> Transposable<T> for Vec<Vec<T>>
where
    T: Clone,
{
    fn transpose(&self) -> Self {
        (self as &[Vec<T>]).transpose()
    }
}

impl<T> Transposable<T> for [Vec<T>]
where
    T: Clone,
{
    fn transpose(&self) -> Vec<Vec<T>> {
        assert!(self.iter().all(|row| row.len() == self[0].len()));
        (0..self[0].len())
            .map(|column| {
                (0..self.len())
                    .map(|row| self[row][column].clone())
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_transpose_vec() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(input.transpose(), expected);
    }

    #[test]
    fn test_transpose_slice() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let slice: &[Vec<i32>] = &input as &[Vec<i32>];
        assert_eq!(slice.transpose(), expected);
    }
}
