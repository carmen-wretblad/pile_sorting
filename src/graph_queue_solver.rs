use crate::board::*;
use crate::board_queue::*;
use crate::graph::*;

pub struct GraphQueueSolverImpl {
    graph: GraphImpl,
    queue: BoardQueueImpl,
}
impl GraphQueueSolverImpl {
    fn new(starting_board: Board) -> Self {
        Self {
            graph: GraphImpl::new(&starting_board),
            queue: BoardQueueImpl::new(starting_board),
        }
    }
    fn next(&mut self) {
        let board: Board = self.queue.next().unwrap(); //TODO
        let children = board.good_children();
        let wanted_children = self.graph.add(board, children);
        self.queue.add(wanted_children);
    }
    fn solved(&self) {
        self.graph.solved();
    }
}
