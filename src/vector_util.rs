use std::{usize, vec};

pub fn contains_zero(input_vector: &Vec<u8>) -> bool {
    let mut vector = input_vector.clone();
    vector.retain(|x| *x == 0);
    !vector.is_empty()
}
pub fn correct_sequence(input_vector: &Vec<u8>) -> bool {
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
    input_vector.len() == all_numbers.len()
}
pub fn all_sequences(lenght: usize) -> Vec<Vec<u8>> {
    let base_vector: Vec<u8> = (1u8..(u8::try_from(lenght + 1).unwrap())).collect();
    recursive_sub_sequences(base_vector)
}

fn recursive_sub_sequences(vec: Vec<u8>) -> Vec<Vec<u8>> {
    assert!(vec.len() > 0);
    if vec.len() == 1 {
        return vec![vec];
    }
    let mut return_vector: Vec<Vec<u8>> = Vec::new();
    for i in 0..vec.len() {
        let head = vec![vec[i]];
        let mut tail = vec.clone();
        tail.remove(i);
        for vector in recursive_sub_sequences(tail) {
            return_vector.push([head.clone(), vector.clone()].concat());
        }
    }
    return_vector
}
pub fn random_vec(lenght: usize) -> Vec<u8> {
    unimplemented!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_zero_true() {
        assert!(contains_zero(&vec![1, 3, 4, 0]));
    }
    #[test]
    fn contains_zero_false() {
        assert!(!contains_zero(&vec![1, 2, 3]));
    }
    #[test]
    fn correct_sequence_true() {
        assert!(correct_sequence(&vec![1, 2, 3, 4, 5]))
    }

    #[test]
    fn correct_sequence_false() {
        assert!(!correct_sequence(&vec![1, 2, 4]));
        assert!(!correct_sequence(&vec![3, 4, 5, 6]));
    }
    #[test]
    fn test_all_sequences() {
        let sequences = all_sequences(3);
        let expected_sequences: Vec<Vec<u8>> = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        for sequence in &sequences {
            assert!(
                expected_sequences.contains(&sequence),
                " got {:?}, expected {:?}",
                &sequences,
                &expected_sequences
            );
        }
        for sequence in &expected_sequences {
            assert!(
                sequences.contains(&sequence),
                "got {:?}, expected {:?}",
                &sequences,
                &expected_sequences
            );
        }
        assert_eq!(sequences.len(), expected_sequences.len());
    }
}
