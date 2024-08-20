use std::collections::HashSet;
use std::usize;

use crate::board::{self, Board};
use crate::board_rep::BoardRep;
use crate::RelMove;
struct Node {
    key: BoardRep,
    parents: Vec<(BoardRep, RelMove)>,
    children: Vec<(BoardRep, RelMove)>,
    nbr_cards: usize,
}

struct ProgramInfo {
    starting_board: Board,
    end_board: Board,
    nbr_piles: usize,
    innitial_nbr_cards: usize,
}

struct NodeHolder {
    info: ProgramInfo,
    steps_taken: usize,
    solved_flag: bool,
    new_generation_board: NewGenerationHolder,
    nodes: HashSet<Node>,
}
impl NodeHolder {
    fn new(board: &Board) -> Self {
        NodeHolder {
            info: ProgramInfo {
                starting_board: board.clone(),
                end_board: Board::new_solved_board(board.piles.len()),
                nbr_piles: board.piles.len(),
                innitial_nbr_cards: board.nbr_cards,
            },
            steps_taken: 0,
            solved_flag: false,
            new_generation_board: NewGenerationHolder {
                generation_status: GenerationStatus::New,
                new_generation: vec![board.clone()],
            },
            nodes: HashSet::new(),
        }
    }

    fn step(&self) {
        self.new_generation();
        self.update_global_heuristic();
        self.remove_childless();
        self.apply_global_heuristic();
        self.remove_unneeded();
    }
    fn new_generation(&self) {
        self.spawn_new_generation();
        self.is_solved();
        self.update_local_heuristic();
        self.prune_new_generation();
        self.generation_shift();
    }
    fn update_global_heuristic(&self) {
        unimplemented!();
    }
    fn update_local_heuristic(&self) {
        unimplemented!();
    }
    fn remove_childless(&self) {
        unimplemented!();
    }
    fn remove_unneeded(&self) {
        unimplemented!();
    }
    fn spawn_new_generation(&self) {
        unimplemented!();
    }
    fn is_solved(&self) {
        unimplemented!();
    }
    fn prune_new_generation(&self) {
        unimplemented!();
    }
    fn generation_shift(&self) {
        unimplemented!();
    }
    fn apply_global_heuristic(&self) {
        unimplemented!();
    }
}

struct NewGenerationHolder {
    generation_status: GenerationStatus,
    new_generation: Vec<Board>,
}
enum GenerationStatus {
    New,
    NewlySpawned,
    Pruned,
    Standard,
    Used,
}
