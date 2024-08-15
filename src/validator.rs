use crate::board::*;
use crate::Move;
use std::collections::hash_map::*;
use std::collections::hash_set::*;
type Solution = Vec<Move>;

fn get_solution(set: &HashSet<Board>, nbr_piles: usize) -> Solution {
    let mut sequence_inverted: Vec<Board> = Vec::new();
    let solution_board = Board::new(&SOLUTION_PILE, nbr_piles);
    let mut board_option: Option<Board> = set.get(&solution_board).cloned();
    loop {
        match board_option {
            Some(board) => {
                sequence_inverted.push(board.clone());
                board_option = board.revert();
            }
            None => {
                break;
            }
        }
    }
    sequence_inverted.reverse();
    board_seq_to_move(&sequence_inverted)
}

fn board_seq_to_move(vec: &Vec<Board>) -> Solution {
    unimplemented!();
}

fn get_all_solutions(map: &HashMap<Board, Vec<Move>>) -> Vec<Solution> {
    unimplemented!();
}
