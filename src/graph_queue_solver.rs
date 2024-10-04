use crate::board::*;
use crate::board_queue::*;
use crate::graph::*;
use crate::validator::*;
use crate::*;

pub struct GraphQueueSolverImpl {
    graph: GraphImpl,
    queue: BoardQueueImpl,
    start: Board,
}
impl GraphQueueSolverImpl {
    pub fn new(starting_board: Board) -> Self {
        Self {
            graph: GraphImpl::new(&starting_board),
            queue: BoardQueueImpl::new(starting_board.clone()),
            start: starting_board,
        }
    }
    pub fn next(&mut self) {
        let board: Board = self.queue.next().unwrap(); //TODO
        let children = board.good_children();
        let wanted_children = self.graph.add(&board, children);
        self.queue.add(wanted_children);
    }
    fn solved(&self) {
        self.graph.solved();
    }
    pub fn get_full_solution(&self) -> Option<RelSolution> {
        let solution = get_solution_graph(self.graph.clone(), &self.start);
        if confirm_solution(&solution, &self.start) {
            Some(solution)
        } else {
            None
        }
    }
    pub fn test(&mut self) {
        let mut boards_seen = 0;
        let mut boards_rejected = 0;
        while !self.graph.found_solution_pile() {
            let board: Board = self.queue.next().unwrap(); //TODO
                                                           //println!("{}", &board);
            let children = board.good_children();
            let boards_given = children.len();
            boards_seen += boards_given;
            let wanted_children = self.graph.add(&board, children);
            boards_rejected += boards_given - wanted_children.len();
            self.queue.add(wanted_children);
        }
        self.graph.test();
        //println!("boards_seen: {boards_seen}");
        //println!("boards_rejected {boards_rejected}");
        /*
        let mut boards_seen_2 = 0;
        let mut boards_rejected_2 = 0;
        while !self.queue.empty() {
            let board: Board = self.queue.next().unwrap(); //TODO
                                                           //println!("{}", &board);
            let children = board.good_children();
            let boards_given = children.len();
            boards_seen_2 += boards_given;
            let wanted_children = self.graph.add(board, children);
            boards_rejected_2 += boards_given - wanted_children.len();
            self.queue.add(wanted_children);
        }*/
        //println!("boards_seen: {boards_seen}");
        //println!("boards_rejected {boards_rejected}");
        //println!("boards_seen_2 {boards_seen_2}");
        //println!("boards_rejeced_2 {boards_rejected_2}");
        //self.graph.test();
    }
}
