use crate::history_tracker::HistoryTracker;
use crate::history_tracker::HistoryTrackerImpl;
use crate::sortedness::Sortedness;
use crate::BoardRep;
//  ##### TODO #######
// Look into the possibility of using mem::swap for replacing values
// Consider tracking higest and lowest card for each pile
// ######
use crate::config::*;
use crate::translator::Translator;
use crate::vector_util;
use crate::AbsMove;
use crate::RelMove;
use core::panic;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::usize;
use std::vec;

pub const SOLUTION_PILE: [u8; 2] = [2, 1];

#[derive(Debug, Clone)]
/// Representation of a full set of cardpiles.
/// Piles are always sorted in order of the value of the bottom card, highest to lowest.
pub struct Board {
    pub piles: Vec<Vec<u8>>,
    pub translator: Translator,
    pub nbr_cards: usize,
    highest_card_on_bottom: bool,
    has_solution_pile: bool,
    pos_highest_card: usize,
    pub history: HistoryTrackerImpl,
    last_shrunk: bool,
}

/// Hashing is based on relative pile positions
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.relative_piles().hash(state);
    }
}
impl Eq for Board {}
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.relative_piles() == other.relative_piles()
    }
}

/// Displays the Board based on relative pile position. A board will look similar to:
/// ```<[5][4][1 2 3]_ _>``` when printed in the terminal
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pile_ids: Vec<usize> = (0..self.piles.len())
            .map(|x| self.translator.into_abs(x))
            .collect();
        write!(f, "({})<", self.nbr_cards)?;
        for i in pile_ids {
            let mut pile = self.piles[i].clone();
            if !pile.is_empty() {
                write!(f, "[")?;
                let last = pile.pop();
                for card in pile.iter() {
                    write!(f, "{} ", card)?;
                }
                if Option::is_some(&last) {
                    write!(f, "{}", last.unwrap())?;
                }
                write!(f, "]")?;
            } else {
                write!(f, " _")?;
            }
        }
        write!(f, ">")?;
        write!(
            f,
            "({}): {} + max({},{})",
            self.theoretical_minimum(),
            self.next_card(),
            self.depth_of_next_card(),
            self.sortedness()
        )
    }
}
fn cartesian_product(nbr: usize) -> Vec<AbsMove> {
    let x = 0..nbr;
    x.clone()
        .flat_map(|y| x.clone().map(move |x| [x.clone(), y.clone()]))
        .filter(|x| x[0] != x[1])
        .collect()
}
impl Board {
    /// Creates a new Board, with all cards placed in the 0th pile.
    pub fn new(pile: &[u8], nbr_piles: usize) -> Board {
        assert!(pile.len() < MAX_NBR_OF_CARDS + 1);
        assert!(MIN_NBR_OF_CARDS < pile.len() + 1);
        assert!(nbr_piles < MAX_NBR_OF_PILES + 1);
        assert!(MIN_NBR_OF_CARDS < nbr_piles + 1);
        assert!(!pile.contains(&0u8));
        assert!(vector_util::correct_sequence(pile));

        let mut new_piles = Vec::new();
        let mut new_nbr_cards = pile.len();
        let mut new_highest_card_on_bottom = false;

        new_piles.push(pile.to_owned());
        for _ in 1..nbr_piles {
            new_piles.push(Vec::<u8>::new());
        }

        if pile.len() == pile[0].into() {
            new_highest_card_on_bottom = true;

            while (new_piles[0][0] == new_piles[0][1] + 1)
                && (new_piles[0][1] == new_piles[0][2] + 1)
            {
                new_piles[0].remove(0);
                new_nbr_cards -= 1;
                if new_nbr_cards == 2 {
                    break;
                }
            }
        }
        let board = Board {
            piles: new_piles,
            translator: Translator::new(nbr_piles),
            nbr_cards: new_nbr_cards,
            highest_card_on_bottom: new_highest_card_on_bottom,
            has_solution_pile: false,
            pos_highest_card: 0,
            history: HistoryTrackerImpl::new(new_nbr_cards),
            last_shrunk: false,
        };
        board
    }
    pub fn new_solved_board(nbr_piles: usize) -> Board {
        Board::new(&[3, 2, 1], nbr_piles) //this will get shrunk to [2,1], which is to small to
                                          //create manually
    }
    pub fn heights(&self) -> Vec<usize> {
        self.piles.iter().map(|x| x.len()).collect()
    }
    pub fn max_height(&self) -> usize {
        *self.heights().iter().max().unwrap()
    }
    /// Gives all moves(absolute) that may be performed that yields a valid state,
    /// performing any other move will cause a panic.
    fn valid_moves_abs(&self) -> Vec<RelMove> {
        let mut empty_piles = Vec::<usize>::new();

        for (i, el) in self.piles.iter().enumerate() {
            if el.is_empty() {
                empty_piles.push(i)
            }
        }
        let mut valid_moves = cartesian_product(self.piles.len());
        valid_moves.retain(|x| !empty_piles.contains(&x[1]) || empty_piles[0] == x[1]);
        valid_moves.retain(|x| !empty_piles.contains(&x[0]));
        valid_moves
    }

    pub fn relative_piles(&self) -> BoardRep {
        self.translator.relative_piles(&self.piles)
    }

    pub fn valid_moves_rel(&self) -> Vec<RelMove> {
        self.translator.into_rel_vector(&self.valid_moves_abs())
    }

    /// Returns all moves(relative) that may lead to a better solution.
    pub fn good_moves_rel(&self) -> Vec<RelMove> {
        if self.solved() {
            return vec![];
        }
        let mut moves = self.valid_moves_abs();
        if self.has_solution_pile {
            let next_card_needed = self.nbr_cards - self.piles[self.pos_highest_card].len();
            for (i, pile) in self.piles.iter().enumerate() {
                if pile
                    .last()
                    .is_some_and(|x| usize::from(*x) == next_card_needed)
                {
                    let move_command = [i, self.pos_highest_card];
                    return vec![self.translator.into_rel_move(move_command)];
                }
                moves.retain(|x| !(x[0] == i && x[1] == self.pos_highest_card));
            }
            moves.retain(|x| x[0] != self.pos_highest_card); // never remove card from solutionpile
        } else {
            moves.retain(|x| x[1] != self.pos_highest_card);
        }
        moves.retain(|x| !self.unnecessary(x));
        moves
            .iter_mut()
            .for_each(|x| *x = self.translator.into_rel_move(*x));
        moves
    }

    fn unnecessary(&self, move_command: &AbsMove) -> bool {
        self.history.unnecessary(*move_command)
    }

    /// Performs a move. Move instructions are "relative".
    pub fn perform_move(&mut self, rel_command: RelMove) {
        let abs_command = self.translator.into_abs_move(rel_command);
        let from_abs = abs_command[0];
        let to_abs = abs_command[1];
        let card = *self.piles[from_abs]
            .last()
            .expect("Should never issue command to take from empty pile");
        let moved_highest_card = usize::from(card) == self.nbr_cards;
        let card_diff = self.nbr_cards - usize::from(card);

        let should_go_on_top =
            (usize::wrapping_sub(self.piles[self.pos_highest_card].len(), card_diff)) == 0;
        let shrink = self.piles[self.pos_highest_card].len() == 2
            && usize::from(self.piles[self.pos_highest_card][0]) == self.nbr_cards
            && usize::from(self.piles[self.pos_highest_card][1]) == self.nbr_cards - 1
            && usize::from(card) == self.nbr_cards - 2;

        self.last_shrunk = shrink;
        if to_abs == self.pos_highest_card {
            if !should_go_on_top {
                self.has_solution_pile = false;
            }
            if shrink {
                self.piles[to_abs].remove(0);
                self.nbr_cards -= 1;
            }
        }
        if moved_highest_card {
            self.pos_highest_card = to_abs;
            self.highest_card_on_bottom = self.piles[to_abs].is_empty();
            self.has_solution_pile = self.piles[to_abs].is_empty();
        }
        self.piles[from_abs].pop().unwrap();
        self.piles[to_abs].push(card);

        self.history.update(abs_command);
        self.translator.update(&self.piles);
    }
    /// A solved pile will be identical to a pile with the cards \[2,1\] in one pile and no other
    /// cards.
    pub fn solved(&self) -> bool {
        self.piles[self.pos_highest_card] == SOLUTION_PILE
    }

    pub fn get_reverted(&self) -> Board {
        match self.history.last_move() {
            None => panic!(),
            Some(mut abs_move) => {
                abs_move.reverse();
                let mut board = self.clone();
                if board.last_shrunk {
                    board.piles[self.pos_highest_card]
                        .insert(0, u8::try_from(self.nbr_cards + 1).unwrap());
                }
                board.perform_move(self.translator.into_rel_move(abs_move));
                board
            }
        }
    }
    pub fn good_children(&self) -> Vec<(Board, RelMove)> {
        let mut children = Vec::new();
        for move_action in self.good_moves_rel() {
            let mut board = self.clone();
            board.perform_move(move_action);
            children.push((board, move_action));
        }
        children
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_board() {
        let input = [4, 3, 2, 1];
        let expected = SOLUTION_PILE;

        let board: Board = Board::new(&input, 4);
        assert_eq!(board.piles[0], expected);
        assert!(board.solved());

        let input = [1, 2, 3, 4];
        let expected = [1, 2, 3, 4];
        let board: Board = Board::new(&input, 4);
        assert_eq!(board.piles[0], expected);
        assert!(!board.solved());

        let input = [8, 7, 6, 5, 1, 2, 3, 4];
        let expected = [6, 5, 1, 2, 3, 4];

        let board = Board::new(&input, 7);
        assert_eq!(board.piles[0], expected);
        assert!(!board.solved())
    }
    #[test]
    #[should_panic]
    fn too_short() {
        Board::new(&[2, 1], 4);
    }
    #[test]
    #[should_panic]
    fn contains_zero() {
        Board::new(&[3, 4, 0, 2, 1], 4);
    }

    #[test]
    #[should_panic]
    fn contains_gap() {
        Board::new(&[1, 2, 3, 5], 4);
    }
    #[test]
    #[should_panic]
    fn starts_at_wrong_index() {
        Board::new(&[2, 3, 4, 6, 5], 4);
    }
    #[test]
    #[should_panic]
    fn too_few_piles() {
        Board::new(&[4, 5, 3, 2, 1], MIN_NBR_OF_PILES - 1);
    }
    #[test]
    #[should_panic]
    fn too_many_piles() {
        Board::new(&[4, 5, 3, 2, 1], MAX_NBR_OF_PILES + 1);
    }

    fn get_hash<T>(obj: &T) -> u64
    where
        T: Hash,
    {
        let mut hasher = fxhash::FxHasher::default();
        obj.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hash_test() {
        //TODO: use a hashmap to double check this

        let mut board1 = Board::new(&[1, 2, 3, 4], 4);
        let mut board2 = Board::new(&[1, 2, 4, 3], 4);

        assert_ne!(get_hash(&board1), get_hash(&board2));

        board1.perform_move([0, 1]); //[4][1,2,3]
        board2.perform_move([0, 1]); //[3][1,2,4]

        assert_ne!(get_hash(&board1), get_hash(&board2));

        board1.perform_move([1, 2]); //[4][3][1,2]
        board2.perform_move([1, 2]); //[4][3][1,2]

        assert_eq!(get_hash(&board1), get_hash(&board2));
    }
    #[test]
    fn hash_set_test() {
        let mut hash_set: HashSet<Board> = HashSet::new();

        let mut board1 = Board::new(&[1, 2, 3, 4], 4);
        let mut board2 = Board::new(&[1, 2, 4, 3], 4);
        insert_new_key_to_hash_set(&mut hash_set, &board1);
        insert_new_key_to_hash_set(&mut hash_set, &board2);

        board1.perform_move([0, 1]); //[4][1,2,3]
        board2.perform_move([0, 1]); //[3][1,2,4]
        insert_new_key_to_hash_set(&mut hash_set, &board1);
        insert_new_key_to_hash_set(&mut hash_set, &board2);

        board1.perform_move([1, 2]); //[4][3][1,2]
        board2.perform_move([1, 2]); //[4][3][1,2]
        insert_new_key_to_hash_set(&mut hash_set, &board1);
        assert!(hash_set.contains(&board2));
    }
    fn insert_new_key_to_hash_set<K>(set: &mut HashSet<K>, key: &K)
    where
        K: std::cmp::Eq + Clone,
        K: std::hash::Hash,
    {
        assert!(!set.contains(key));
        set.insert(key.clone());
        assert!(set.contains(key));
    }

    #[test]
    fn display_test() {
        let mut board1 = Board::new(&[1, 2, 3, 4], 4);
        let mut board2 = Board::new(&[1, 2, 4, 3], 4);

        assert_ne!(format!("{}", board1), format!("{}", board2));
        println!("{board1} != {board2} ");

        board1.perform_move([0, 1]); //[4][1,2,3]
        board2.perform_move([0, 1]); //[3][1,2,4]

        assert_ne!(format!("{}", board1), format!("{}", board2));
        println!("{board1} != {board2} ");

        board1.perform_move([1, 2]); //[4][3][1,2]
        board2.perform_move([1, 2]); //[4][3][1,2]

        assert_eq!(format!("{}", board1), format!("{}", board2));
        println!("{board1} == {board2} ");
    }
    #[test]
    fn valid_solutions_test() {
        let board1 = Board::new(&[1, 2, 3, 4, 5], 4);
        let board2 = Board::new(&[1, 5, 2, 3, 4], 4);
        let expected = vec![[0, 1]];
        assert_eq!(board1.valid_moves_abs(), expected);
        assert_eq!(board1.valid_moves_rel(), expected);
        assert_eq!(board2.valid_moves_abs(), expected);
        assert_eq!(board2.valid_moves_rel(), expected);
    }
    #[test]
    fn hashing_double_take() {
        let mut all_board: Vec<Board> = Vec::new();
        for pile in vector_util::all_sequences(5) {
            all_board.push(Board::new(&pile, 4));
        }
        let mut set: HashSet<Board> = HashSet::new();
        for board in &all_board {
            assert!(!set.contains(&board));
            set.insert(board.clone());
            assert!(set.contains(&board));
            assert_eq!(set.get(&board).unwrap(), board);
        }
        for board in &all_board {
            assert!(set.contains(&board));
            assert_eq!(&set.get(&board).unwrap(), &board);
        }
        for mut board in all_board {
            board.perform_move([0, 1]);
            assert!(!set.contains(&board));
            set.insert(board.clone());
            assert!(set.contains(&board));
        }
    }
    #[test]
    fn partial_equality_test() {
        let pile1 = [1, 4, 3, 2];
        let pile2 = [1, 4, 2, 3];
        let mut board1 = Board::new(&pile1, 4);
        let mut board2 = Board::new(&pile2, 4);
        let board3 = Board::new(&pile1, 5);
        assert!(board1 == board1);
        assert!(board2 == board2);
        assert!(board3 == board3);
        assert!(board1 != board2);
        assert!(board1 != board3);

        board1.perform_move([0, 1]); // [2] [1,4,3]
        board2.perform_move([0, 1]); // [3] [1,4,2]

        board1.perform_move([1, 2]); // [3][2][1,4]
        board2.perform_move([1, 2]); // [3][2][1,4]

        assert!(board1 == board2);

        board1.perform_move([1, 0]); // [3,2][1,4]
        assert!(board1 != board2);
        assert!(board1 == board1);
    }
}
