use core::panic;
use std::{u8, usize};
// TODO:
// figure out to hash it
// display
// no "pastmove"
// should moves have usize or u8?
use crate::Move;
pub struct Board {
    pub piles: Vec<Vec<u8>>,
    pub abs_to_rel_translator: Vec<usize>,
    pub nbr_cards: usize,
    pub solution_pile_pos: Option<usize>, // make bool
}
impl Board {
    pub fn new(pile: &Vec<u8>, nbr_piles: usize) -> Board {
        assert!(pile.len() > 2);
        assert!(nbr_piles > 2);

        let mut new_piles = Vec::new();
        let mut new_position_translator = Vec::new();
        let mut new_nbr_cards = pile.len();
        let mut new_solution_pile_pos = None;

        new_piles.push(pile.clone());
        for _ in 1..nbr_piles {
            new_piles.push(Vec::<u8>::new());
        }

        for i in 0..nbr_piles {
            new_position_translator.push(i);
        }

        if pile.len() == pile[0].into() {
            new_solution_pile_pos = Some(0);
        }

        while (new_piles[0][0] == new_piles[0][1] + 1) && (new_piles[0][1] == new_piles[0][2] + 1) {
            new_piles[0].remove(0);
            new_nbr_cards -= 1;
            if new_nbr_cards == 2 {
                break;
            }
        }
        Board {
            piles: new_piles,
            abs_to_rel_translator: new_position_translator,
            nbr_cards: new_nbr_cards,
            solution_pile_pos: new_solution_pile_pos,
        }
    }
    fn valid_moves(&self) -> Vec<Move> {
        let mut non_empty_piles = Vec::<usize>::new();
        let mut empty_piles = Vec::<usize>::new();

        for (i, el) in self.piles.iter().enumerate() {
            if el.is_empty() {
                empty_piles.push(i);
            } else {
                non_empty_piles.push(i);
            }
        }
        // doesn't take from empty pile
        // doesn't put in empty pile except first one
        // doesn't take from one pile and put into same
        unimplemented!();
    }

    fn good_moves(&self) -> Vec<Move> {
        // does'nt put back card it just grabbed
        // doesn't move cards away from solutionpile
        // doesn't put bad cards on solutionpile (???)
        // always put good card on solutionpile
        unimplemented!();
    }
    fn perform_move(&mut self, move_command: Move) {
        // seperate into move and place logic?

        let from_rel = move_command[0];
        let to_rel = move_command[1];
        let from_abs = self.rel_to_abs(from_rel);
        let to_abs = self.rel_to_abs(to_rel);

        assert!(!self.piles[from_abs].is_empty());
        assert!(self.valid_moves().contains(&move_command));
        let card = self.piles[from_abs].pop().unwrap();
        self.piles[to_abs].push(card);
        if self.piles[to_abs].len() == 1 || (self.piles[from_abs].len() == 0) {
            self.update_indexes();
        }
        if usize::from(card) == self.nbr_cards && self.piles[to_abs].len() == 1 {
            self.solution_pile_pos = Some(usize::from(card));
        }

        if to_rel == 0 && Option::is_some(&self.solution_pile_pos) {
            if (self.piles[to_abs].len() == 3) && (self.piles[to_abs][1] == card + 1) {
                self.piles[to_abs].remove(0);
                self.nbr_cards -= 1;
            } else {
                self.solution_pile_pos = None;
            }
        }
    }

    fn solved(&self) -> bool {
        match self.solution_pile_pos {
            Some(pile_nbr) => self.piles[pile_nbr] == vec![2, 1], //is
            //this enought?
            None => false,
        }
    }
    fn abs_to_rel(&self, abs_val: usize) -> usize {
        self.abs_to_rel_translator[abs_val]
    }

    fn rel_to_abs(&self, rel_val: usize) -> usize {
        for (i, el) in self.abs_to_rel_translator.iter().enumerate() {
            if el == &rel_val {
                return i;
            }
        }
        panic!();
    }
    fn update_indexes(&mut self) {
        unimplemented!();
        // order rel based on highest card
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn new_board() {
        {
            let input = vec![4, 3, 2, 1];
            let expected = vec![2, 1];

            let board: Board = Board::new(&input, 4);
            assert_eq!(board.piles[0], expected);
        }
        {
            let input = vec![1, 2, 3, 4];
            let expected = vec![1, 2, 3, 4];

            let board: Board = Board::new(&input, 4);
            assert_eq!(board.piles[0], expected)
        }
        {
            let input = vec![8, 7, 6, 5, 1, 2, 3, 4];
            let expected = vec![6, 5, 1, 2, 3, 4];

            let board = Board::new(&input, 7);
            assert_eq!(board.piles[0], expected)
        }
    }
}
