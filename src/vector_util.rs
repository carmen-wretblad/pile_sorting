use std::{usize, vec};

pub fn contains_zero(input_vector: Vec<u8>) -> bool {
    let mut vector = input_vector;
    vector.retain(|x| *x == 0);
    !vector.is_empty()
}
pub fn correct_sequence(input_vector: Vec<u8>) -> bool {
    let mut all_numbers: Vec<u8> = Vec::new();
    for number in 1..input_vector.len() + 1 {
        all_numbers.push(u8::try_from(number).unwrap());
    }
    for number in all_numbers.iter() {
        if !input_vector.contains(&number) {
            return false;
        }
    }
    for number in input_vector.iter() {
        if !all_numbers.contains(&number) {
            return false;
        }
    }
    true
}
pub fn all_sequences(lenght: usize) -> Vec<Vec<u8>> {
    unimplemented!();
}
pub fn random_vec(lenght: usize) -> Vec<u8> {
    unimplemented!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_zero_true() {
        assert!(contains_zero(vec![1, 3, 4, 0]));
    }
    #[test]
    fn contains_zero_false() {
        assert!(!contains_zero(vec![1, 2, 3]));
    }
    #[test]
    fn correct_sequence_true() {
        assert!(correct_sequence(vec![1, 2, 3, 4, 5]))
    }

    #[test]
    fn correct_sequence_false() {
        assert!(!correct_sequence(vec![1, 2, 4]));
        assert!(!correct_sequence(vec![3, 4, 5, 6]));
    }
}
