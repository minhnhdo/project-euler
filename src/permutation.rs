struct Permutation<T> {
    is_first_iter: bool,
    indices: Vec<usize>,
    data: Vec<T>,
}

impl<T> Permutation<T> {
    fn new(data: Vec<T>) -> Self {
        Permutation {
            is_first_iter: true,
            indices: (0..=data.len() - 1).collect(),
            data,
        }
    }
}

impl<T> Iterator for Permutation<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indices.len() == 0 {
            return None;
        }

        if self.is_first_iter {
            self.is_first_iter = false;
            return Some(self.data.clone());
        }

        // find last peak
        let mut i = self.indices.len() - 1;
        while i > 0 && self.indices[i - 1] >= self.indices[i] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        // find index to permute
        let mut j = self.indices.len() - 1;
        while self.indices[j] <= self.indices[i - 1] {
            j -= 1;
        }

        self.indices.swap(i - 1, j);
        self.indices[i..].reverse();

        Some(self.indices.iter().map(|i| self.data[*i].clone()).collect())
    }
}

pub fn permutations<T>(data: Vec<T>) -> impl Iterator<Item = Vec<T>>
where
    T: Clone,
{
    Permutation::new(data)
}

#[cfg(test)]
mod tests {
    use super::permutations;
    #[test]
    fn test_permutations() {
        assert_eq!(
            permutations(vec![1, 2, 3, 4]).collect::<Vec<_>>(),
            vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 4, 3],
                vec![1, 3, 2, 4],
                vec![1, 3, 4, 2],
                vec![1, 4, 2, 3],
                vec![1, 4, 3, 2],
                vec![2, 1, 3, 4],
                vec![2, 1, 4, 3],
                vec![2, 3, 1, 4],
                vec![2, 3, 4, 1],
                vec![2, 4, 1, 3],
                vec![2, 4, 3, 1],
                vec![3, 1, 2, 4],
                vec![3, 1, 4, 2],
                vec![3, 2, 1, 4],
                vec![3, 2, 4, 1],
                vec![3, 4, 1, 2],
                vec![3, 4, 2, 1],
                vec![4, 1, 2, 3],
                vec![4, 1, 3, 2],
                vec![4, 2, 1, 3],
                vec![4, 2, 3, 1],
                vec![4, 3, 1, 2],
                vec![4, 3, 2, 1],
            ]
        );
    }
}
