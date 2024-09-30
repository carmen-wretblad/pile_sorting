use crate::board::*;
use crate::BoardRep;

use crate::RelMove;
use crate::RelSolution;

pub trait Graph {
    fn new(starting: Board) -> Self;
    fn add_node(&self, parent: Board, child: (Board, RelMove)) -> Board;
    fn add(&mut self, parent: Board, children: Vec<(Board, RelMove)>) -> Vec<Board>;
}

pub trait GraphInfo: Graph {
    fn nbr_nodes(&self) -> usize;
    fn nbr_edges(&self) -> usize;
}

pub trait SolvableGraph: Graph {
    fn solved(&self) -> bool;
    fn solution(&self) -> Option<RelSolution>;
    /// Not required to be exhaustive
    fn solutions(&self) -> Option<Vec<RelSolution>>;
}
pub trait DebugGraph: Graph {
    fn all_solutions(&self) -> Option<Vec<RelSolution>>;
    fn shortest_path(&self) -> Option<Vec<RelSolution>>;
    fn longest_path(&self) -> Option<Vec<RelSolution>>;
}
pub struct GraphImpl {}
impl Graph for GraphImpl {
    fn new(starting: Board) -> Self {
        unimplemented!();
    }
    fn add_node(&self, rep: Board, child: (Board, RelMove)) -> Board {
        unimplemented!();
    }
    fn add(&mut self, parent: Board, children: Vec<(Board, RelMove)>) -> Vec<Board> {
        unimplemented!();
    }
}
impl SolvableGraph for GraphImpl {
    fn solved(&self) -> bool {
        unimplemented!()
    }
    fn solution(&self) -> Option<RelSolution> {
        unimplemented!()
    }
    fn solutions(&self) -> Option<Vec<RelSolution>> {
        unimplemented!()
    }
}

impl GraphImpl {
    fn contains(&self, board: &Board) -> bool {
        unimplemented!()
    }
    fn add_edge(&mut self) {
        unimplemented!()
    }
    fn add_node_internal(&mut self) {
        unimplemented!()
    }
    fn want_info(&self, board: &Board) -> bool {
        self.contains(board) //TODO
    }
}
