#![allow(unused, dead_code)]
use crate::board::*;
use crate::validator::*;
use crate::*;
//use std::collections::HashSet;
use fxhash::*;

#[derive(Debug)]
pub enum MoveChoice {
    Valid,
    Good,
}

pub struct BFS {
    strategy: MoveChoice,
    name: String,
    starting_board: Board,
    found_boards: FxHashSet<Board>,
    next_boards: FxHashSet<Board>,
    current_boards: FxHashSet<Board>,
    pub step_counter: usize,
    solved_board: Option<Board>,
    pub board_counter: usize,
}
impl BFS {
    pub fn new(board: &Board, strategy: MoveChoice) -> Self {
        let mut bfs = BFS {
            strategy,
            name: "BFS".to_string(),
            starting_board: board.clone(),
            next_boards: FxHashSet::default(),
            current_boards: FxHashSet::default(),
            found_boards: FxHashSet::default(),
            step_counter: 0,
            solved_board: None,
            board_counter: 0,
        };
        bfs.current_boards.insert(bfs.starting_board.clone());
        bfs
    }
    fn get_selected_moveset(&self, board: &Board) -> Vec<RelMove> {
        match &self.strategy {
            MoveChoice::Valid => board.valid_moves_rel(),
            MoveChoice::Good => board.good_moves_rel(),
        }
    }
    pub fn internal_step(&mut self) -> bool {
        for board in &self.current_boards {
            self.found_boards.insert(board.clone());
            for move_command in self.get_selected_moveset(board) {
                let mut newboard = board.clone();
                newboard.perform_move(move_command);
                if newboard.solved() {
                    self.found_boards.insert(newboard.clone());
                    if BFS_SHOULD_PRINT_FOUND_BOARDS {
                        println!("board: {}", &newboard);
                    }
                    self.solved_board = Some(newboard.clone());
                    return true;
                }
                if !self.found_boards.contains(&newboard) {
                    if BFS_SHOULD_PRINT_FOUND_BOARDS {
                        println!("board :{}", &newboard)
                    };
                    self.next_boards.insert(newboard.clone());
                    self.board_counter += 1;
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
        if BFS_SHOULD_PRINT_STEP_COUNTER {
            println!("step {} ################################################################################", self.step_counter);
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
    use super::*;
    use crate::vector_util;
    #[test]
    fn compare_good_and_valid() {
        let mut all_board: Vec<Board> = Vec::new();
        for pile in vector_util::all_sequences(5) {
            all_board.push(Board::new(&pile));
        }
        all_board.push(Board::new(&vec![2, 5, 3, 4, 6, 1, 7]));
        for board in all_board {
            best_solution_not_exluded(&board);
        }
    }

    fn best_solution_not_exluded(board: &Board) {
        let mut bfs_good = BFS::new(&board, MoveChoice::Good);
        let good_len = bfs_good
            .solve()
            .expect("There should always be a good solution")
            .len();

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
    #[test]
    fn valid_bfs_works() {
        for pile in vector_util::all_sequences(5) {
            assert!(
                vector_util::correct_sequence(&pile),
                "pile {:?} is a valid pile",
                &pile
            );
            let board = Board::new(&pile);
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
            let board = Board::new(&pile);
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
        let strategy = MoveChoice::Good;
        let board = Board::new(&pile);
        let mut bfs = BFS::new(&board, strategy);
        let potential_solution = bfs.solve();
    }
}
