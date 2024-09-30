use std::collections::HashMap;
use std::usize;

use crate::board::*;
use crate::sortedness::*;
use crate::BoardRep;
use crate::RelMove;
use crate::RelSolution;
use petgraph::algo::astar;
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
    fn test(&self);
    /// Not required to be exhaustive
    fn solutions(&self) -> Option<Vec<RelSolution>>;
}
pub trait DebugGraph: Graph {
    fn all_solutions(&self) -> Option<Vec<RelSolution>>;
    fn shortest_path(&self) -> Option<Vec<RelSolution>>;
    fn longest_path(&self) -> Option<Vec<RelSolution>>;
}
#[derive(Debug)]
pub struct GraphImpl {
    underlying_structure: PGraph<BoardRep, RelMove>,
    seen_nodes: HashMap<BoardRep, NodeIndex>,
    board_rep_for_index: HashMap<NodeIndex, Board>,
    index_starting_pile: Option<NodeIndex>,
    index_solution_pile: Option<NodeIndex>,
}
impl Graph for GraphImpl {
    fn new(starting: &Board) -> Self {
        let mut graph = Self {
            underlying_structure: PGraph::<BoardRep, RelMove>::new(),
            seen_nodes: HashMap::new(),
            board_rep_for_index: HashMap::new(),
            index_starting_pile: None,
            index_solution_pile: None,
        };
        let index: NodeIndex = graph
            .underlying_structure
            .add_node(starting.relative_piles());
        graph.seen_nodes.insert(starting.relative_piles(), index);
        graph.index_starting_pile = Some(index);
        graph.board_rep_for_index.insert(index, starting.clone());
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
        self.index_solution_pile.is_some()
    }
    fn solved(&self) -> bool {
        unimplemented!()
    }
    fn solution(&self) -> Option<RelSolution> {
        unimplemented!()
    }
    fn test(&self) {
        let path = astar(
            &self.underlying_structure,
            self.index_starting_pile.unwrap(),
            |x| self.board_rep_for_index.get(&x).unwrap().solved(),
            |_| 1,
            |x| {
                self.board_rep_for_index
                    .get(&x)
                    .unwrap()
                    .theoretical_minimum()
            },
        )
        .unwrap();

        println!("{}", path.1.len());
        println!("{}", path.0);
        //for item in path.1 {
        //    println!("{}", self.board_rep_for_index.get(&item).unwrap());
        //}

        //for item in path.1 {
        //    println!("{}", self.board_rep_for_index.get(&item).unwrap());
        //}
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
        let index: NodeIndex = self.underlying_structure.add_node(board.relative_piles());
        if board.solved() {
            self.index_solution_pile = Some(index);
        }
        self.seen_nodes.insert(board.relative_piles(), index);
        self.board_rep_for_index.insert(index, board.clone());
    }
    fn want_info(&self, board: &Board) -> bool {
        !self.contains(board) //TODO
    }
}
