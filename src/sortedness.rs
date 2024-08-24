use crate::BoardRep;
use std::usize;

pub fn sortendess_boardrep(board_rep: &BoardRep) -> usize {
    sortedness_vector_list(&boardrep_to_piles(board_rep))
}

pub fn max_height_boardrep(board_rep: &BoardRep) -> usize {
    max_height(&boardrep_to_piles(board_rep))
}

fn boardrep_to_piles(vector: &[u8]) -> Vec<Vec<u8>> {
    let mut piles: Vec<Vec<u8>> = Vec::new();
    let mut holder: Vec<u8> = Vec::new();
    for el in vector {
        match el {
            200 => holder.clear(),
            222 => piles.push(holder.clone()),
            _ => holder.push(*el),
        }
    }
    piles
}

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

fn max_height(vectors: &[Vec<u8>]) -> usize {
    vectors.iter().map(|w| w.len()).max().unwrap()
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
    #[test]
    fn boardrep_to_piles_test() {
        let start = 200u8;
        let end = 222u8;
        let vec: &[u8] = &[start, 4, 2, end, start, 3, 1, end];
        let result: Vec<Vec<u8>> = boardrep_to_piles(vec);
        let expected: Vec<Vec<u8>> = (&[[4, 2].to_vec(), [3, 1].to_vec()]).to_vec();
        assert_eq!(result, expected);
    }
    #[test]
    fn heights_test() {
        let piles = &[[4, 5, 2].to_vec(), [6, 1].to_vec(), [7, 8, 9, 10].to_vec()];
        assert_eq!(max_height(piles), 4);
    }
}
