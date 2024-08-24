use std::iter::Sum;
use std::usize;

use crate::board::Board;
use crate::BoardRep;

pub fn sortedness_vector(vector: &[u8]) -> usize {
    let differences: Vec<i16> = vector
        .windows(2)
        .map(|w| i16::from(w[0]) - i16::from(w[1]))
        .collect();
    let mut sign_changes: Vec<bool> = differences
        .windows(2)
        .map(|w| (w[0] < 0) != (w[1] < 0))
        .collect();
    sign_changes.retain(|w| *w);
    sign_changes.len()
}
fn sortedness_vector_list(vectors: &[Vec<u8>]) -> usize {
    let mut agg = 0;
    for vec in vectors {
        agg += sortedness_vector(vec);
    }
    agg
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sortedness_valid() {
        assert_eq!(sortedness_vector(&[1, 2, 3]), 0);
        assert_eq!(sortedness_vector(&[3, 2, 1]), 0);
        assert_eq!(sortedness_vector(&[2, 3, 1]), 1);
        assert_eq!(sortedness_vector(&[1, 3, 2]), 1);
        assert_eq!(sortedness_vector(&[5, 4, 6, 2, 3, 1]), 4);
        assert_eq!(
            sortedness_vector_list(&[
                [3, 2, 1].to_vec(),
                [2, 3, 1].to_vec(),
                [5, 4, 6, 2, 3, 1].to_vec()
            ]),
            5
        );
    }
}
