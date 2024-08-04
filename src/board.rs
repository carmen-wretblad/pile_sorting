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
    used_piles: u8,
}
impl Board {
    fn new(pile: &Vec<u8>, nbr_piles: u8) -> Board {
        debug_assert!(pile.len() > 1);
        debug_assert!(nbr_piles > 2);

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

        while new_piles[0][0] == new_piles[0][1] + 1 {
            new_piles[0].remove(0);
            new_nbr_cards = new_nbr_cards - 1;
            if new_nbr_cards == 1 {
                break;
            }
        }
        Board {
            piles: new_piles,
            position_translator: new_position_translator,
            nbr_cards: u8::try_from(new_nbr_cards).unwrap(),
            solution_pile_pos: new_solution_pile_pos,
            used_piles: 1,
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
        unimplemented!()
    }
}
