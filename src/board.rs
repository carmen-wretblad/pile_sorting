use core::panic;
use std::{u8, usize};
// TODO:
// figure out to hash it
// display
// no "pastmove"
// should moves have usize or u8?
// Look into the possibility of using mem::swap for replacing values
// Abs vs Rel moves
// const solutionpile
use crate::Move;
#[derive(Debug)]
/// Representation of a full set of cardpiles.
/// Piles are always sorted in order of the value of the bottom card, highest to lowest.
pub struct Board {
    pub piles: Vec<Vec<u8>>,
    abs_to_rel_translator: Vec<usize>,
    pub nbr_cards: usize,
    solution_pile_pos: Option<usize>, // make bool
    last_move: Option<Move>,
}
impl Board {
    /// Creates a new Board, with all cards placed in the 0th pile.
    pub fn new(pile: &[u8], nbr_piles: usize) -> Board {
        assert!(pile.len() > 2);
        assert!(nbr_piles > 2);

        let mut new_piles = Vec::new();
        let mut new_position_translator = Vec::new();
        let mut new_nbr_cards = pile.len();
        let mut new_solution_pile_pos = None;

        new_piles.push(pile.to_owned());
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
            last_move: None,
        }
    }
    /// Gives all moves(absolute) that may be performed that yields a valid state,
    /// performing any other move will cause a panic.
    pub fn valid_moves_abs(&self) -> Vec<Move> {
        // TODO: make this more legible?
        let mut non_empty_piles = Vec::<usize>::new();
        let mut empty_piles = Vec::<usize>::new();

        for (i, el) in self.piles.iter().enumerate() {
            if el.is_empty() {
                empty_piles.push(i);
            } else {
                non_empty_piles.push(i);
            }
        }

        let valid_from = non_empty_piles.clone();
        let mut valid_to = non_empty_piles.clone();
        if !empty_piles.is_empty() {
            valid_to.push(*empty_piles.first().unwrap());
        }
        let mut valid_moves = Vec::<Move>::new();
        for from in &valid_from {
            for to in &valid_to {
                valid_moves.push([*from, *to])
            }
        }

        // doesn't take from empty pile
        // doesn't put in empty pile except first one
        // doesn't take from one pile and put into same
        valid_moves
    }
    fn valid_moves_rel(&self) -> Vec<Move> {
        let mut moves = self.valid_moves_abs();
        moves.iter_mut().for_each(|x| *x = self.abs_to_rel_move(*x));
        moves
    }

    /// Returns all moves(relative) that may lead to a better solution.
    pub fn good_moves_rel(&self) -> Vec<Move> {
        assert!(!self.solved());
        let mut valid_moves = self.valid_moves_abs();
        assert!(!valid_moves.is_empty());
        valid_moves.retain(|x| self.not_last_move(x)); // you never need to undo the last move.
        valid_moves.retain(|x| x[0] != x[1]); /* picking up and putting down a card in the same
                                              // place is meaningless */

        match &self.solution_pile_pos {
            Some(pile_pos) => {
                for (i, el) in self.piles.iter().enumerate() {
                    if usize::from(el[el.len() - 1]) == self.nbr_cards - 2 {
                        return vec![[i, 0]]; /* if we can put the next card for the solutionpile is
                                             // exposed, putting it on the solutionpile is the only logical move */
                    }
                }
                valid_moves.retain(|x| x[0] != *pile_pos) //we never want to remove cards from a
                                                          //solution-pile
            }
            None => (),
        }

        /* Speculated but not implemented: doesn't put bad cards on solutionpile.
        not sure if there are cases where such a reshuffle is required or not */
        assert!(!valid_moves.is_empty());
        valid_moves
            .iter_mut()
            .for_each(|x| *x = self.abs_to_rel_move(*x));
        valid_moves
    }
    fn not_last_move(&self, move_command: &Move) -> bool {
        if Option::is_none(&self.last_move) {
            return true;
        }
        let last_move = self.last_move.unwrap();
        if (move_command[0] == last_move[1]) && (move_command[1] == last_move[0]) {
            return false;
        }
        true
    }

    /// Performs a move. Move instructions are "relative".
    pub fn perform_move(&mut self, move_command: Move) {
        // seperate into move and place logic?

        let from_rel = move_command[0];
        let to_rel = move_command[1];
        let from_abs = self.rel_to_abs(from_rel);
        let to_abs = self.rel_to_abs(to_rel);

        self.last_move = Some([from_abs, to_abs]);

        assert!(!self.piles[from_abs].is_empty());
        assert!(
            self.valid_moves_rel().contains(&move_command),
            "move command {:?}, wasn't contained in valid commands: {:?} (rel) || {:?} (abs)",
            move_command,
            self.valid_moves_rel(),
            self.valid_moves_abs(),
        );

        let card = self.piles[from_abs].pop().unwrap();
        self.piles[to_abs].push(card);
        if self.piles[to_abs].len() == 1 || (self.piles[from_abs].is_empty()) {
            self.update_indexes();
        }

        if usize::from(card) == self.nbr_cards && self.piles[to_abs].len() == 1 {
            self.solution_pile_pos = Some(usize::from(card));
        }

        if to_rel == 0
            && Option::is_some(&self.solution_pile_pos)
            && (self.piles[to_abs].len() == 3)
            && (self.piles[to_abs][1] == card + 1)
        {
            self.piles[to_abs].remove(0);
            self.nbr_cards -= 1;
        }
    }
    /// A solved pile will be identical to a pile with the cards \[2,1\] in one pile and no other
    /// cards.
    pub fn solved(&self) -> bool {
        match self.solution_pile_pos {
            Some(pile_nbr) => self.piles[pile_nbr] == vec![2, 1], //is
            //this enought?
            None => false,
        }
    }
    fn abs_to_rel(&self, abs_val: usize) -> usize {
        self.abs_to_rel_translator[abs_val]
    }
    fn abs_to_rel_move(&self, abs_move: Move) -> Move {
        [self.abs_to_rel(abs_move[0]), self.abs_to_rel(abs_move[1])]
    }

    fn rel_to_abs(&self, rel_val: usize) -> usize {
        for (i, el) in self.abs_to_rel_translator.iter().enumerate() {
            if el == &rel_val {
                return i;
            }
        }
        panic!();
    }
    fn rel_to_abs_move(&self, rel_move: Move) -> Move {
        [self.rel_to_abs(rel_move[0]), self.rel_to_abs(rel_move[1])]
    }

    fn update_indexes(&mut self) {
        let mut non_empty_piles = Vec::<usize>::new();
        let mut empty_piles = Vec::<usize>::new();

        for (i, el) in self.piles.iter().enumerate() {
            if el.is_empty() {
                empty_piles.push(i);
            } else {
                non_empty_piles.push(i);
            }
        }
        non_empty_piles.sort_by(|a, b| self.piles[*b][0].cmp(&self.piles[*a][0]));
        let mut counter = 0;
        for pile in &non_empty_piles {
            self.abs_to_rel_translator[*pile] = counter;
            counter += 1;
        }
        for pile in empty_piles {
            self.abs_to_rel_translator[pile] = counter;
            counter += 1;
        }
        if usize::from(self.piles[non_empty_piles[0]][0]) == self.nbr_cards {
            self.solution_pile_pos = Some(self.rel_to_abs(0));
        } else {
            self.solution_pile_pos = None
        }
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
    #[test]
    fn printing_test() {
        let input = vec![1, 2, 3, 4, 5];
        let mut board = Board::new(&input, 4);
        println!("{:?}", &board);
        println!("{:?}", board.valid_moves_abs());
        board.perform_move([0, 1]);
        println!("{:?}", &board);
        board.perform_move([1, 0]);
        println!("{:?}", &board);
        board.perform_move([1, 2]);
        println!("{:?}", &board);
        board.perform_move([1, 0]);
        println!("{:?}", &board);
        board.perform_move([1, 0]);
        println!("{:?}", &board);
        board.perform_move([1, 0]);
        println!("{:?}", &board);
        println!("{:?}", board.solved())
    }
}
