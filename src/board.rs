use core::panic;
use std::{u8, usize};
// TODO:
// figure out to hash it
// display
// no "pastmove"
// should moves have usize or u8?
use crate::Move;
#[derive(Debug)]
pub struct Board {
    pub piles: Vec<Vec<u8>>,
    pub abs_to_rel_translator: Vec<usize>,
    pub nbr_cards: usize,
    pub solution_pile_pos: Option<usize>, // make bool
}
impl Board {
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

        let valid_moves = valid_moves
            .into_iter()
            .filter(|x| x[0] != x[1])
            .map(|x| self.abs_to_rel_move(x));

        // doesn't take from empty pile
        // doesn't put in empty pile except first one
        // doesn't take from one pile and put into same
        Vec::from_iter(valid_moves)
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
        assert!(
            self.valid_moves().contains(&move_command),
            "move command {:?}, wasn't contained in valid commands: {:?}",
            move_command,
            self.valid_moves()
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
        println!("{:?}", board.valid_moves());
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
