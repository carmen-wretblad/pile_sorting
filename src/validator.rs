use crate::board::*;
use crate::sortedness::Sortedness;
use crate::RelMove;
pub type RelSolution = Vec<RelMove>;
use crate::history_tracker::*;
use std::collections::HashSet;
const VALIDATOR_SHOULD_PRINT: bool = true;

pub fn get_solution(set: &HashSet<Board>, starting_board: &Board) -> RelSolution {
    let nbr_piles = starting_board.piles.len();
    let mut board_sequence_inverted: Vec<Board> = Vec::new();
    let solution_board_proxy = Board::new_solved_board(nbr_piles);
    let solution_board: Board = set
        .get(&solution_board_proxy)
        .expect("There must be a solved board for this to work")
        .to_owned();
    let mut next_board = solution_board;
    loop {
        board_sequence_inverted.push(next_board.clone());
        if next_board == *starting_board {
            break;
        }

        let future_next_board = set
            .get(&next_board.get_reverted())
            .expect("Always have to be a next board")
            .clone();
        assert_ne!(future_next_board, next_board);
        next_board = future_next_board;
    }

    board_sequence_inverted.reverse();
    board_seq_to_move(&board_sequence_inverted)
}

pub fn board_seq_to_move(vec: &Vec<Board>) -> RelSolution {
    let mut vec = vec.to_owned();
    assert!(vec[0].history_tracker.last_move().is_none());
    vec.remove(0);
    vec.into_iter()
        .map(|x| x.history_tracker.last_move().unwrap())
        .collect()
}

pub fn confirm_solution(solution: &RelSolution, starting_board: &Board) -> bool {
    let mut board = starting_board.clone();
    if VALIDATOR_SHOULD_PRINT {
        println!("starting board validation for: {}", &board);
    }
    let mut stepper = solution.len();
    for abs_move_command in solution {
        let rel_move = board.translator.into_rel_move(*abs_move_command);
        board.perform_move(rel_move, "confirming_solution");
        if VALIDATOR_SHOULD_PRINT {
            stepper -= 1;

            //println!("{}", &board);
            //println!("ordering {:?}", board.order_object());
            println!(
                "step: ({stepper}) theory: {}, board: {}",
                board.theoretical_minimum(),
                board
            );
        }
    }
    board.solved()
}
