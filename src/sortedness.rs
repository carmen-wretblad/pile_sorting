use crate::BoardRep;
use std::{u8, usize};

trait Sortedness {
    fn heights(&self) -> Vec<usize>;
    fn max_height(&self) -> usize {
        *self
            .heights()
            .iter()
            .max()
            .expect("Can't create empty boards")
    }
    fn sortedness(&self) -> usize;
    fn nbr_cards(&self) -> usize;
    fn has_solution_pile(&self) -> bool;
    fn next_card(&self) -> u8;
    fn depth_of_card(&self, card: u8) -> Option<usize>;
    fn depth_of_next_card(&self) -> Option<usize> {
        self.depth_of_card(self.next_card())
    }
}
type Piles = Vec<Vec<u8>>;

impl Sortedness for Piles {
    fn heights(&self) -> Vec<usize> {
        self.iter().map(|w| w.len()).collect()
    }
    fn sortedness(&self) -> usize {
        let mut agg = 0;
        for vec in self {
            agg += sortedness_vector(vec);
        }
        agg
    }
    fn nbr_cards(&self) -> usize {
        self.iter().map(|w| w.len()).sum()
    }
    fn has_solution_pile(&self) -> bool {
        usize::from(self[0][0]) == self.nbr_cards()
    }
    fn next_card(&self) -> u8 {
        let nbr_cards: u8 =
            u8::try_from(self.nbr_cards()).expect("Can't have more cards than fit in an u8");
        if !self.has_solution_pile() {
            return nbr_cards;
        }
        if self[0].len() < 2 || self[0][1] != nbr_cards - 1 {
            return nbr_cards - 1;
        }
        nbr_cards - 2
    }
    fn depth_of_card(&self, card: u8) -> Option<usize> {
        for pile in self {
            match pile.iter().position(|x| *x == card) {
                Some(position) => return Some(pile.len() - (position + 1)),
                None => (),
            }
        }
        None
    }
}

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

fn nbr_cards(piles: &Vec<Vec<u8>>) -> usize {
    piles.iter().map(|w| w.len()).sum()
}

fn has_solution_pile(piles: &Vec<Vec<u8>>) -> bool {
    usize::from(piles[0][0]) == nbr_cards(piles)
}
fn depth_of_card(piles: &Vec<Vec<u8>>, card: u8) -> Option<usize> {
    for pile in piles {
        match pile.iter().position(|x| *x == card) {
            Some(position) => return Some(pile.len() - (position + 1)),
            None => (),
        }
    }
    None
}
fn next_card(piles: &Vec<Vec<u8>>) -> u8 {
    let nbr_cards: u8 = u8::try_from(nbr_cards(piles)).unwrap();
    if !has_solution_pile(piles) {
        return nbr_cards;
    }
    if piles[0].len() < 2 || piles[0][1] != nbr_cards - 1 {
        return nbr_cards - 1;
    }
    nbr_cards - 2
}
fn depth_of_next_card(piles: &Vec<Vec<u8>>) -> Option<usize> {
    let nbr_cards: u8 = u8::try_from(nbr_cards(piles)).unwrap();
    let next_card = next_card(piles);
    depth_of_card(piles, next_card)
}

#[cfg(test)]
mod tests {
    use crate::sortedness::{depth_of_card, has_solution_pile};

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
    #[test]
    fn nbr_cards_test() {
        let piles = ([[4, 5, 2].to_vec(), [6, 1].to_vec(), [7, 8, 9, 10].to_vec()]).to_vec();
        assert_eq!(nbr_cards(&piles), 9);
    }
    #[test]
    fn has_solution_pile_test() {
        let piles_true = ([[5, 4].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert!(has_solution_pile(&piles_true));

        let piles_false = ([[4, 5].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert!(!has_solution_pile(&piles_false));
    }
    #[test]
    fn depth_of_card_test() {
        let piles = ([[4, 5, 2].to_vec(), [6, 1].to_vec(), [7, 8, 9, 10].to_vec()]).to_vec();
        assert_eq!(depth_of_card(&piles, 8), Some(2));
    }
    #[test]
    fn next_card_test() {
        let piles = ([[5, 4].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert_eq!(next_card(&piles), 3);
    }
    #[test]
    fn depth_of_next_card_test() {
        let piles = ([[5, 4].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert_eq!(depth_of_next_card(&piles), Some(0));
    }
}
