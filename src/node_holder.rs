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
    new_generation: Vec<(Board, NodeContent)>,
    future_generation: Vec<(Board, NodeContent)>,
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
            new_generation: vec![(board.clone(), NodeContent::new())], //todo
            future_generation: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn step(&mut self) {
        self.spawn_future_generation();
        if self.check_solved() {
            self.solved_flag = true;
        }
        let nbr_cards = self.get_local_heuristic();
        self.prune_future_generation(nbr_cards);
        self.remove_local_childless();
        //self.remove_local_unneeded();
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
        let mut board_to_keep = Vec::new();
        for (new_board, new_content) in &self.new_generation {
            for (child, _) in &new_content.children.get_items() {
                if self.future_generation_contains(&child) {
                    board_to_keep.push(new_board.clone());
                }
            }
        }
        self.new_generation
            .retain(|item| board_to_keep.contains(&item.0));
    }
    fn remove_local_unneeded(&mut self) {
        unimplemented!();
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

    fn generation_shift(&mut self) {
        for node in &self.new_generation {
            println!("inserting board {}", node.0);
            self.nodes.insert(node.0.relative_piles(), node.1.clone());
        }
        self.new_generation.clear();
        let _ = self
            .future_generation
            .iter()
            .map(|item| self.new_generation.push(item.clone()));

        self.future_generation.clear();
    }
    fn spawn_future_generation(&mut self) {
        for (board, content) in &mut self.new_generation {
            let children = board.good_children();
            for (child, move_performed) in children {
                content
                    .children
                    .add_item(&(child.relative_piles(), move_performed));

                let mut new_content = NodeContent::new();
                new_content
                    .parents
                    .add_item(&(board.relative_piles(), move_performed));
                self.future_generation.push((child, new_content))
            }
        }
    }

    fn future_generation_contains(&self, boardrep: &BoardRep) -> bool {
        for (future_board, _) in &self.future_generation {
            if boardrep == &future_board.relative_piles() {
                return true;
            }
        }
        false
    }
}
