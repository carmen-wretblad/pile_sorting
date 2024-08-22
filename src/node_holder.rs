use crate::board;
#[allow(unused, dead_code)]
use crate::board::Board;
use crate::node_content::NodeContent;
use crate::BoardRep;
use crate::RelMove;
use std::collections::HashMap;
use std::usize;

struct ProgramInfo {
    starting_board: Board,
    end_board: Board,
    nbr_piles: usize,
    innitial_nbr_cards: usize,
}

pub struct NodeHolder {
    info: ProgramInfo,
    steps_taken: usize,
    solved_flag: bool,
    new_generation: Vec<(Board, RelMove)>,
    future_generation: Vec<(Board, RelMove)>,
    nodes: HashMap<BoardRep, NodeContent>,
}
impl NodeHolder {
    pub fn new(board: &Board) -> Self {
        Self {
            info: ProgramInfo {
                starting_board: board.clone(),
                end_board: Board::new_solved_board(board.piles.len()),
                nbr_piles: board.piles.len(),
                innitial_nbr_cards: board.nbr_cards,
            },
            steps_taken: 0,
            solved_flag: false,
            new_generation: vec![(board.clone(), [0, 0])], //todo
            future_generation: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn step(&mut self) {
        self.spawn_new_generation();
        if self.check_solved() {
            self.solved_flag = true;
        }
        let nbr_cards = self.get_local_heuristic();
        self.prune_future_generation(nbr_cards);
        //self.remove_local_childless();
        //self.remove_local_unneeded();
        //self.move_local_to_nodes();
        self.generation_shift();
    }
    fn get_local_heuristic(&self) -> usize {
        let mut local_heuristic = usize::MAX;
        for (board, _) in &self.future_generation {
            if board.nbr_cards < local_heuristic {
                local_heuristic = board.nbr_cards
            }
        }
        local_heuristic
    }
    fn remove_local_childless(&mut self) {
        unimplemented!();
    }
    fn remove_local_unneeded(&mut self) {
        unimplemented!();
    }
    fn spawn_new_generation(&mut self) {
        for (board, rel_move) in &self.new_generation {
            for (new_board, new_move) in board.good_children() {
                self.future_generation.push((new_board, new_move));
            }
        }
    }
    fn check_solved(&self) -> bool {
        for (new_board, _) in &self.future_generation {
            if new_board.solved() {
                return true;
            }
        }
        false
    }
    pub fn is_solved(&self) -> bool {
        self.solved_flag
    }

    fn prune_future_generation(&mut self, nbr_cards: usize) {
        self.future_generation
            .retain(|x| x.0.nbr_cards == nbr_cards);
    }
    fn move_local_to_nodes(&mut self) {
        unimplemented!();
    }

    fn generation_shift(&mut self) {
        for (board, rel_move) in &self.new_generation {
            println!("inserting board {}", board);
            self.nodes
                .insert(board.relative_piles(), NodeContent::new());
        }
        self.new_generation = self.future_generation.clone();
        self.future_generation.clear();
    }
}
