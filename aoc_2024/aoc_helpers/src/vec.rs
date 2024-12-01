pub trait Rotatable<T> {
    fn rot90(&self) -> Vec<Vec<T>>;
    fn rot270(&self) -> Vec<Vec<T>>;
    fn rot180(&self) -> Vec<Vec<T>>;
}

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

impl<T> Rotatable<T> for Vec<Vec<T>>
where
    T: Clone,
{
    fn rot90(&self) -> Self {
        (self as &[Vec<T>]).rot90()
    }

    fn rot270(&self) -> Self {
        (self as &[Vec<T>]).rot270()
    }

    fn rot180(&self) -> Self {
        (self as &[Vec<T>]).rot180()
    }
}

impl<T> Rotatable<T> for [Vec<T>]
where
    T: Clone,
{
    fn rot90(&self) -> Vec<Vec<T>> {
        assert!(self.iter().all(|row| row.len() == self[0].len()));
        (0..self[0].len())
            .map(|original_x| {
                (0..self.len())
                    .map(|original_y| self[self.len() - original_y - 1][original_x].clone())
                    .collect()
            })
            .collect()
    }

    fn rot270(&self) -> Vec<Vec<T>> {
        assert!(self.iter().all(|row| row.len() == self[0].len()));
        (0..self[0].len())
            .map(|original_x| {
                (0..self.len())
                    .map(|original_y| self[original_y][self[0].len() - original_x - 1].clone())
                    .collect()
            })
            .collect()
    }

    fn rot180(&self) -> Vec<Vec<T>> {
        assert!(self.iter().all(|row| row.len() == self[0].len()));
        self.iter()
            .rev()
            .map(|line| line.iter().rev().cloned().collect())
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

    #[test]
    fn test_rot90() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![4, 1], vec![5, 2], vec![6, 3]];
        let slice: &[Vec<i32>] = &input as &[Vec<i32>];
        assert_eq!(slice.rot90(), expected);
    }

    #[test]
    fn test_rot270() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![3, 6], vec![2, 5], vec![1, 4]];
        let slice: &[Vec<i32>] = &input as &[Vec<i32>];
        assert_eq!(slice.rot270(), expected);
    }

    #[test]
    fn test_rot180() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![6, 5, 4], vec![3, 2, 1]];
        let slice: &[Vec<i32>] = &input as &[Vec<i32>];
        assert_eq!(slice.rot180(), expected);
    }
}
