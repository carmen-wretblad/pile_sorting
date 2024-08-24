use std::iter::Sum;

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

#[cfg(test)]
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
    }
}
