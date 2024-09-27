#[allow(unused, dead_code)]
use crate::board::Board;
use crate::node_content::NodeContent;
use crate::sortedness::Sortedness;
use crate::BoardRep;
use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;

struct ProgramInfo {
    starting_board: Board,
    end_board: Board,
    nbr_piles: usize,
    innitial_nbr_cards: usize,
}

pub struct NodeHolder {
    info: ProgramInfo,
    solved_flag: bool,
    new_generation: Vec<(Board, NodeContent)>,
    future_generation: Vec<(Board, NodeContent)>,
    nodes: HashMap<BoardRep, NodeContent>,
    max_height_previous: usize,
    pub board_counter: usize,
    pub steps: usize,
    past_minimum: usize,
    pub bad_boards: HashSet<BoardRep>,
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
            solved_flag: false,
            new_generation: vec![(board.clone(), NodeContent::new())], //todo
            future_generation: Vec::new(),
            nodes: HashMap::new(),
            max_height_previous: board.nbr_cards,
            board_counter: 0,
            steps: 0,
            past_minimum: usize::MAX,
            bad_boards: HashSet::new(),
        }
    }

    pub fn step(&mut self) {
        self.spawn_future_generation();
        if self.check_solved() {
            self.solved_flag = true;
        } else {
            self.prune_future_generation();
            self.remove_local_childless();
            self.generation_shift();
            self.steps += 1;
        }
    }
    fn remove_local_childless(&mut self) {
        let mut board_to_keep = Vec::new();
        for (new_board, new_content) in &self.new_generation {
            for (child, _) in &new_content.children.get_items() {
                if self.future_generation_contains(child) {
                    board_to_keep.push(new_board.clone());
                }
            }
        }
        self.new_generation
            .retain(|item| board_to_keep.contains(&item.0));
    }
    fn check_solved(&self) -> bool {
        for (new_board, _) in &self.future_generation {
            if new_board.solved() {
                println!("solved! {}", &new_board);
                return true;
            }
        }
        false
    }
    pub fn is_solved(&self) -> bool {
        self.solved_flag
    }

    fn prune_future_generation(&mut self) {
        println!("before removing: {}", self.future_generation.len());

        self.future_generation
            .retain(|x| x.0.theoretical_minimum() < self.past_minimum + 1);

        println!("after removing:  {}", self.future_generation.len());
    }

    fn generation_shift(&mut self) {
        for node in &self.new_generation {
            self.nodes.insert(node.0.relative_piles(), node.1.clone());
            self.board_counter += 1;
        }
        self.new_generation.clear();
        for future_board in &self.future_generation {
            self.new_generation.push(future_board.clone())
        }

        self.future_generation.clear();
    }
    fn spawn_future_generation(&mut self) {
        let new_generation_boards_reps: Vec<Vec<u8>> = self
            .new_generation
            .iter()
            .map(|x| x.0.relative_piles())
            .collect();
        for (board, content) in &mut self.new_generation {
            let children = board.good_children();
            for (child, move_performed) in children {
                if !new_generation_boards_reps.contains(&child.relative_piles())
                    && !self.nodes.contains_key(&child.relative_piles())
                {
                    content
                        .children
                        .add_item(&(child.relative_piles(), move_performed));

                    let mut new_content = NodeContent::new();
                    new_content
                        .parents
                        .add_item(&(board.relative_piles(), move_performed));
                    if child.max_height() < self.max_height_previous {
                        self.max_height_previous = child.max_height();
                    }
                    self.future_generation.push((child, new_content));
                }
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
