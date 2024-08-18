use crate::board::*;
use crate::program::MoveChoice;
use crate::RelMove;
pub type RelSolution = Vec<RelMove>;
use std::collections::HashSet;
//use indexmap::HashSet;
const VALIDATOR_SHOULD_PRINT: bool = false;

pub fn get_solution(
    set: &HashSet<Board>,
    starting_board: &Board,
    strategy_used: &MoveChoice,
) -> RelSolution {
    let nbr_piles = starting_board.piles.len();
    let mut board_sequence_inverted: Vec<Board> = Vec::new();
    let solution_board_proxy = Board::new_solved_board(nbr_piles);
    let solution_board: Board = set
        .get(&solution_board_proxy)
        .expect("There must be a solved board for this to work")
        .to_owned();
    let mut next_board = solution_board;
    loop {
        //println!("next board is {}", next_board);
        board_sequence_inverted.push(next_board.clone());
        if next_board == starting_board.to_owned() {
            break;
        }

        let future_next_board = set
            .get(&next_board.revert())
            .expect("Always have to be a next board")
            .clone();
        assert_ne!(future_next_board, next_board);
        next_board = future_next_board;
    }

    board_sequence_inverted.reverse();
    /* assert!(board_sequence_inverted.last().unwrap().solved());
    assert!(
        board_sequence_inverted[0] == *starting_board,
        "expected starting board {}, but got {} \n
        move strategy: {:?}",
        starting_board,
        board_sequence_inverted[0],
        strategy_used
    ); */
    board_seq_to_move(&board_sequence_inverted)
}

pub fn board_seq_to_move(vec: &Vec<Board>) -> RelSolution {
    let mut vec = vec.to_owned();
    assert!(vec[0].last_move.is_none());
    vec.remove(0);
    vec.into_iter().map(|x| x.last_move.unwrap()).collect()
}

pub fn confirm_solution(solution: &RelSolution, starting_board: &Board) -> bool {
    let mut board = starting_board.clone();
    if VALIDATOR_SHOULD_PRINT {
        //println!("starting board validation for: {}", &board);
    }
    for abs_move_command in solution {
        let rel_move = board.abs_to_rel_move(*abs_move_command);
        board.perform_move(rel_move, "confirming_solution");
        //board.perform_move_unchecked(rel_move);

        //board.abs_to_rel_translator = starting_board.abs_to_rel_translator.clone();
        if VALIDATOR_SHOULD_PRINT {
            println!("{}", &board);
        }
    }
    board.solved()
}
