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
                end_board: Board::new_solved_board(),
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
            let nbr_cards = self.get_local_heuristic();
            self.prune_future_generation(nbr_cards);
            //self.remove_local_childless();
            self.generation_shift();
            self.steps += 1;
        }
    }
    fn get_local_heuristic(&mut self) -> usize {
        let mut local_nbr_cards = usize::MAX;
        let mut local_max_height = usize::MIN;
        let mut minimum = usize::MAX;
        let mut maximum = usize::MIN;
        for (board, _) in &self.future_generation {
            if board.nbr_cards < local_nbr_cards {
                local_nbr_cards = board.nbr_cards
            }

            if board.max_height() > local_max_height {
                local_max_height = board.max_height()
            }
            if board.theoretical_minimum() < minimum {
                minimum = board.theoretical_minimum();
            }
            if board.theoretical_minimum() > maximum {
                maximum = board.theoretical_minimum();
            }
        }
        self.max_height_previous = local_max_height;
        println!("maximum: {maximum}");
        println!("minimum: {minimum}");
        self.past_minimum = minimum;
        local_nbr_cards
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

    fn prune_future_generation(&mut self, _nbr_cards: usize) {
        println!("before removing: {}", self.future_generation.len());
        //self.future_generation
        //    .retain(|x| x.0.nbr_cards == nbr_cards);

        self.future_generation
            .retain(|x| x.0.theoretical_minimum() < self.past_minimum + 1);

        /* if self.future_generation.len() < 10000 {
        } else {
            self.future_generation
                .sort_by(|a, b| a.0.order_object().cmp(&b.0.order_object()));
            self.future_generation.drain(10000..);
        } */
        println!("after removing:  {}", self.future_generation.len());
    }
    /*fn prune_new_generation(&mut self) {
        println!("before removing new: {}", self.future_generation.len());
        let mut list_of_stuff_to_keep: Vec<Board> = Vec::new();
        for (board, content) in &self.new_generation {
            if self.future_generation_contains_some(&content.get_children()) {
                list_of_stuff_to_keep.push(board.clone());
            }
        }
        self.new_generation
            .retain(|x| list_of_stuff_to_keep.contains(&x.0));
        println!("after removing new:  {}", self.future_generation.len());
    } */

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
                //&& !self.bad_boards.contains(&child.relative_piles())
                //&& (child.max_height() <= self.max_height_previous
                //    || self.steps < self.info.innitial_nbr_cards / 2)
                //&& ((child.max_height() + self.steps) < self.info.innitial_nbr_cards <-- bad
                //    || self.steps < self.info.innitial_nbr_cards / 3)
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
    fn future_generation_contains_some(&self, boardreps: &Vec<BoardRep>) -> bool {
        for (future_board, _) in &self.future_generation {
            for board_rep in boardreps {
                if board_rep == &future_board.relative_piles() {
                    return true;
                }
            }
        }
        false
    }
    fn new_generation_contains(&self, boardrep: &BoardRep) -> bool {
        for (new_board, _) in &self.new_generation {
            if boardrep == &new_board.relative_piles() {
                return true;
            }
        }
        false
    }
}
