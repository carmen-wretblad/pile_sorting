use std::collections::HashMap;

use crate::board::*;
use crate::BoardRep;
use crate::RelMove;
use crate::RelSolution;
use petgraph::graph::Graph as PGraph;
use petgraph::graph::NodeIndex;

pub trait Graph {
    fn new(starting: &Board) -> Self;
    fn add_node(&mut self, parent: &Board, child: (Board, RelMove)) -> Option<Board>;
    fn add(&mut self, parent: Board, children: Vec<(Board, RelMove)>) -> Vec<Board>;
}

pub trait GraphInfo: Graph {
    fn nbr_nodes(&self) -> usize;
    fn nbr_edges(&self) -> usize;
}

pub trait SolvableGraph: Graph {
    fn found_solution_pile(&self) -> bool;
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
pub struct GraphImpl {
    underlying_structure: PGraph<(), RelMove>,
    seen_nodes: HashMap<BoardRep, NodeIndex>,
}
impl Graph for GraphImpl {
    fn new(starting: &Board) -> Self {
        let mut graph = Self {
            underlying_structure: PGraph::<(), RelMove>::new(),
            seen_nodes: HashMap::new(),
        };
        graph.add_node_internal(starting);
        graph
    }

    fn add_node(&mut self, rep: &Board, child: (Board, RelMove)) -> Option<Board> {
        if self.contains(&child.0) {
            None
        } else {
            self.add_node_internal(&child.0);
            self.add_edge(&rep, &child.0, child.1);
            Some(child.0)
        }
    }
    fn add(&mut self, parent: Board, children: Vec<(Board, RelMove)>) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        for child in children {
            let result = self.add_node(&parent, child);
            if let Some(board) = result {
                boards.push(board);
            }
        }
        boards
    }
}
impl SolvableGraph for GraphImpl {
    fn found_solution_pile(&self) -> bool {
        unimplemented!()
    }
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
        self.seen_nodes.contains_key(&board.relative_piles())
    }
    fn add_edge(&mut self, from: &Board, to: &Board, edge: RelMove) {
        let from_index = self.seen_nodes.get(&from.relative_piles()).unwrap();
        let to_index = self.seen_nodes.get(&to.relative_piles()).unwrap();
        self.underlying_structure
            .add_edge(*from_index, *to_index, edge);
    }
    //TODO, make sure & works
    fn add_node_internal(&mut self, board: &Board) {
        let index: NodeIndex = self.underlying_structure.add_node(());
        self.seen_nodes.insert(board.relative_piles(), index);
    }
    fn want_info(&self, board: &Board) -> bool {
        !self.contains(board) //TODO
    }
}
