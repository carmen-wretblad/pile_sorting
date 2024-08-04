use std::u8;

use crate::Move;
// TODO:
// figure out to hash it
// display
// no "pastmove"
pub struct Board {
    pub piles: Vec<Vec<u8>>,
    pub position_translator: Vec<u8>,
    pub nbr_cards: u8,
    pub solution_pile_pos: Option<u8>,
    pub non_empty_piles: u8,
}
impl Board {
    pub fn new(pile: &Vec<u8>, nbr_piles: u8) -> Board {
        assert!(pile.len() > 2);
        assert!(nbr_piles > 2);

        let mut new_piles = Vec::new();
        new_piles.push(pile.clone());
        let mut new_position_translator = Vec::new();

        for _ in 1..nbr_piles {
            new_piles.push(Vec::<u8>::new());
        }
        for i in 0..nbr_piles {
            new_position_translator.push(i);
        }
        let mut new_nbr_cards = pile.len();
        let mut new_solution_pile_pos = None;

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
            position_translator: new_position_translator,
            nbr_cards: u8::try_from(new_nbr_cards).unwrap(),
            solution_pile_pos: new_solution_pile_pos,
            non_empty_piles: 1,
        }
    }
    fn valid_moves(&self) -> Vec<u8> {
        unimplemented!();
    }

    fn good_moves(&self) -> Vec<u8> {
        unimplemented!();
    }
    fn perform_move(&mut self, move_command: Move) -> Board {
        unimplemented!();
    }
    fn solved(&self) -> bool {
        (self.non_empty_piles == 0) && (self.piles[0] == vec![2, 1])
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
