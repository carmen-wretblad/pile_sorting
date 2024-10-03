use crate::{board::Board, BoardRep};
use std::cmp::{max, Ord};
use std::fmt::Debug;

pub trait Sortedness: Debug {
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
    fn depth_of_next_card(&self) -> usize {
        self.depth_of_card(self.next_card()).expect(
            format!(
                "next card must always have a position \n 
                    next card should be: {}
                    given the piles {:?}",
                self.next_card(),
                &self,
            )
            .as_str(),
        )
    }
    fn order_object(&self) -> OrderObject {
        OrderObject {
            next_card: self.next_card(),
            depth_of_next_card: self.depth_of_next_card(),
            sortedness: self.sortedness(),
        }
    }
    fn theoretical_minimum(&self) -> usize {
        usize::from(self.next_card()) + max(self.depth_of_next_card(), self.sortedness())
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct OrderObject {
    next_card: u8,
    depth_of_next_card: usize,
    sortedness: usize,
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
        if card == 0 {
            return Some(0);
        }
        for pile in self {
            if let Some(position) = pile.iter().position(|x| *x == card) {
                return Some(pile.len() - (position) - 1);
            }
        }
        None
    }
}

impl Sortedness for BoardRep {
    fn heights(&self) -> Vec<usize> {
        boardrep_to_piles(self).heights()
    }
    fn sortedness(&self) -> usize {
        boardrep_to_piles(self).sortedness()
    }
    fn nbr_cards(&self) -> usize {
        boardrep_to_piles(self).nbr_cards()
    }
    fn has_solution_pile(&self) -> bool {
        boardrep_to_piles(self).has_solution_pile()
    }
    fn next_card(&self) -> u8 {
        boardrep_to_piles(self).next_card()
    }
    fn depth_of_card(&self, card: u8) -> Option<usize> {
        boardrep_to_piles(self).depth_of_card(card)
    }
}

impl Sortedness for Board {
    fn heights(&self) -> Vec<usize> {
        self.relative_piles().heights() // unnecessary indirection TODO
    }
    fn sortedness(&self) -> usize {
        self.relative_piles().sortedness() // unnecessary indirection TODO
    }
    fn nbr_cards(&self) -> usize {
        self.nbr_cards
    }
    fn has_solution_pile(&self) -> bool {
        self.relative_piles().has_solution_pile() // unnecessary indirection TODO
    }
    fn next_card(&self) -> u8 {
        self.relative_piles().next_card() // unnecessary indirection TODO
    }
    fn depth_of_card(&self, card: u8) -> Option<usize> {
        self.relative_piles().depth_of_card(card) // unnecessary indirection TODO
    }
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

        let piles: Piles = vec![
            [3, 2, 1].to_vec(),
            [2, 3, 1].to_vec(),
            [5, 4, 6, 2, 4, 1].to_vec(),
        ];
        assert_eq!(piles.sortedness(), 5);
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
        let piles: Piles = vec![[4, 5, 2].to_vec(), [6, 1].to_vec(), [7, 8, 9, 10].to_vec()];
        assert_eq!(piles.max_height(), 4);
    }
    #[test]
    fn nbr_cards_test() {
        let piles: Piles = ([[4, 5, 2].to_vec(), [6, 1].to_vec(), [7, 8, 9, 10].to_vec()]).to_vec();
        assert_eq!(piles.nbr_cards(), 9);
        let piles: Vec<Vec<u8>> = [
            [1, 9, 5, 6, 3, 8, 4, 7, 12, 2, 16].to_vec(),
            [10, 15, 14, 13].to_vec(),
            [11].to_vec(),
            [].to_vec(),
            [].to_vec(),
        ]
        .to_vec();
        assert_eq!(piles.nbr_cards(), 16);
        let vec = [1, 9, 5, 6, 3, 8, 4, 7, 12, 2, 16, 11, 13, 14, 15, 10].to_vec();
        let mut board = Board::new(&vec);
        board.perform_move([0, 1]); //10
        board.perform_move([1, 0]); //15
        board.perform_move([1, 0]); //14
        board.perform_move([1, 0]); //13
        board.perform_move([1, 2]); //11
        assert_eq!(board.nbr_cards, 16);
        assert_eq!(board.nbr_cards(), 16);
        assert_eq!(board.relative_piles().nbr_cards(), 16);
    }
    #[test]
    fn has_solution_pile_test() {
        let piles_true: Piles = ([[5, 4].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert!(piles_true.has_solution_pile());

        let piles_false = ([[4, 5].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert!(!piles_false.has_solution_pile());
    }
    #[test]
    fn depth_of_card_test() {
        let piles: Piles = ([[4, 5, 2].to_vec(), [6, 1].to_vec(), [7, 8, 9, 10].to_vec()]).to_vec();
        assert_eq!(piles.depth_of_card(8), Some(2));
    }
    #[test]
    fn next_card_test() {
        let piles: Piles = ([[5, 4].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert_eq!(piles.next_card(), 3);
    }
    #[test]
    fn depth_of_next_card_test() {
        let piles: Piles = ([[5, 4].to_vec(), [2, 1, 3].to_vec()]).to_vec();
        assert_eq!(piles.depth_of_next_card(), 0);
    }
}
