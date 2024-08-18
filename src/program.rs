#![allow(unused)]
const SHOULD_PRINT_FOUND_BOARDS: bool = false;
const SHOULD_PRINT_STEP_COUNTER: bool = false;
use std::thread::current;

use crate::validator::*;
use crate::{board, validator};
use crate::{board::*, AbsMove, RelMove};
//use indexmap::IndexSet;
use std::collections::HashSet;
/*pub trait Program: Iterator {
    fn starting_state(&self) -> &Board;
    fn done(&self) -> bool;
    /// Runs the program until a new Move has been reached, must change result of done method when
    /// applicable
    fn step(&mut self) -> Option<Move>;
    /// Runs the program to completion.
    /// Will return new moves made only
    fn run(&mut self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::<Move>::new();
        while (!self.done()) {
            if let Some(value) = self.step() {
                vec.push(value)
            }
        }
        vec
    }
    /// Returns all moves so far
    fn progress(&self) -> &Vec<Move>;
} */

#[derive(Debug)]
pub enum MoveChoice {
    Valid,
    Good,
    Unconfirmed,
}

pub struct BFS {
    strategy: MoveChoice,
    name: String,
    starting_board: Board,
    found_boards: HashSet<Board>,
    next_boards: HashSet<Board>,
    current_boards: HashSet<Board>,
    step_counter: usize,
    solved_board: Option<Board>,
}
impl BFS {
    pub fn new(board: &Board, strategy: MoveChoice) -> Self {
        let mut bfs = BFS {
            strategy: strategy,
            name: "BFS".to_string(),
            starting_board: board.clone(),
            next_boards: HashSet::new(),
            current_boards: HashSet::new(),
            found_boards: HashSet::new(),
            step_counter: 0,
            solved_board: None,
        };
        bfs.current_boards.insert(bfs.starting_board.clone());
        bfs
    }
    fn get_selected_moveset(&self, board: &Board) -> Vec<RelMove> {
        match &self.strategy {
            MoveChoice::Valid => board.valid_moves_rel(),
            MoveChoice::Good => board.good_moves_rel(),
            MoveChoice::Unconfirmed => board.unconfirmed_validity_moves_rel(),
        }
    }
    pub fn internal_step(&mut self) -> bool {
        for board in &self.current_boards {
            self.found_boards.insert(board.clone());
            for move_command in self.get_selected_moveset(board) {
                let mut newboard = board.clone();
                newboard.perform_move(
                    move_command,
                    &format!("bfs with srategy {:?}", self.strategy),
                );
                if newboard.solved() {
                    self.found_boards.insert(newboard.clone());
                    if SHOULD_PRINT_FOUND_BOARDS {
                        println!("{}", &newboard);
                    }
                    self.solved_board = Some(newboard.clone());
                    return true;
                }
                if !self.found_boards.contains(&newboard) {
                    if SHOULD_PRINT_FOUND_BOARDS {
                        println!("{}", &newboard)
                    };
                    self.next_boards.insert(newboard.clone());
                }
            }
        }
        assert!(
            !self.next_boards.is_empty(),
            "Next board is empty \n starting board is: {}",
            self.starting_board
        );
        self.current_boards = self.next_boards.clone();
        self.next_boards.clear();
        self.step_counter += 1;
        if SHOULD_PRINT_STEP_COUNTER {
            println!("step {}", self.step_counter);
        }
        assert!(self.solved_board.is_none());
        false
    }
    pub fn get_full_solution(&self) -> Option<RelSolution> {
        let solution = validator::get_solution(&self.found_boards, &self.starting_board);
        if confirm_solution(&solution, &self.starting_board) {
            Some(solution)
        } else {
            None
        }
    }
    pub fn solve(&mut self) -> Option<RelSolution> {
        while !self.internal_step() {}
        self.get_full_solution()
    }
}

#[cfg(test)]
mod test {
    use crate::vector_util;
    enum CompareTo {
        Valid,
        Unconfirmed,
    }
    use super::*;
    #[test]
    fn compare_good_and_valid() {
        let mut all_board: Vec<Board> = Vec::new();
        for pile in vector_util::all_sequences(5) {
            all_board.push(Board::new(&pile, 3));
        }
        all_board.push(Board::new(&vec![2, 5, 3, 4, 6, 1, 7], 4));
        for board in all_board {
            best_solution_not_exluded(&board, CompareTo::Valid);
        }
    }
    #[test]
    fn compare_good_and_unconfirmed() {
        let mut all_board: Vec<Board> = Vec::new();
        for pile in vector_util::all_sequences(5) {
            all_board.push(Board::new(&pile, 4));
        }
        all_board.push(Board::new(&vec![2, 5, 3, 4, 6, 1, 7], 4));
        for board in all_board {
            best_solution_not_exluded(&board, CompareTo::Unconfirmed);
        }
    }

    fn best_solution_not_exluded(board: &Board, compare_to: CompareTo) {
        let mut bfs_good = BFS::new(&board, MoveChoice::Good);
        let good_len = bfs_good
            .solve()
            .expect("There should always be a good solution")
            .len();

        match compare_to {
            CompareTo::Valid => {
                let mut bfs_valid = BFS::new(&board, MoveChoice::Valid);
                let valid_len = bfs_valid
                    .solve()
                    .expect("There is always a valid solution")
                    .len();
                assert!(
                    valid_len == good_len,
                    "valid: {}, good: {}",
                    valid_len,
                    good_len,
                );
            }
            CompareTo::Unconfirmed => {
                let mut bfs_unconfirmed = BFS::new(&board, MoveChoice::Unconfirmed);
                let unconfirmed_len = bfs_unconfirmed
                    .solve()
                    .expect("There is always a solution")
                    .len();
                assert!(
                    unconfirmed_len == good_len,
                    "uconfirmed: {}, good: {}",
                    unconfirmed_len,
                    good_len,
                )
            }
        }
    }
    #[test]
    fn valid_bfs_works() {
        for pile in vector_util::all_sequences(5) {
            assert!(
                vector_util::correct_sequence(&pile),
                "pile {:?} is a valid pile",
                &pile
            );
            let board = Board::new(&pile, 5);
            let mut bfs_valid = BFS::new(&board, MoveChoice::Valid);
            bfs_valid
                .solve()
                .expect("There is always a valid solution")
                .len();
        }
    }
    #[test]
    fn good_bfs_works() {
        for pile in vector_util::all_sequences(5) {
            assert!(
                vector_util::correct_sequence(&pile),
                "pile {:?} is a valid pile",
                &pile
            );
            let board = Board::new(&pile, 5);
            let mut bfs_valid = BFS::new(&board, MoveChoice::Good);
            bfs_valid
                .solve()
                .expect("There is always a valid solution")
                .len();
        }
    }
    #[test]
    fn confirming_bug() {
        let pile = vec![1, 5, 2, 3, 4];
        let nbr_piles = 4;
        let strategy = MoveChoice::Good;
        let board = Board::new(&pile, nbr_piles);
        let mut bfs = BFS::new(&board, strategy);
        let potential_solution = bfs.solve();
        //assert!(bfs.solved_board.unwrap() == Board::new_solved_board(4))
    }
}
