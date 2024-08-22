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

struct NodeHolder {
    info: ProgramInfo,
    steps_taken: usize,
    solved_flag: bool,
    new_generation: Vec<(Board, RelMove)>,
    future_generation: Vec<(Board, RelMove)>,
    nodes: HashMap<BoardRep, NodeContent>,
}
impl NodeHolder {
    fn new(board: &Board) -> Self {
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

    fn step(&mut self) {
        self.new_generation();
        //self.update_global_heuristic();
        //self.remove_childless();
        //self.apply_global_heuristic();
        //self.remove_unneeded();
    }
    fn new_generation(&mut self) {
        self.spawn_new_generation();
        if self.check_solved() {
            self.solved_flag = true;
        }
        let nbr_cards = self.get_local_heuristic();
        self.prune_future_generation(nbr_cards);
        self.generation_shift();
    }
    fn update_global_heuristic(&mut self) {
        unimplemented!();
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
    fn remove_childless(&mut self) {
        unimplemented!();
    }
    fn remove_unneeded(&mut self) {
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
    fn prune_future_generation(&mut self, nbr_cards: usize) {
        self.future_generation
            .retain(|x| x.0.nbr_cards == nbr_cards);
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
    fn apply_global_heuristic(&mut self) {
        unimplemented!();
    }
}
