use std::usize;

use crate::{board::Board, sortedness::Sortedness};
use priority_queue::priority_queue::PriorityQueue;

pub trait BoardQueue {
    fn add_single(&mut self, board: Board);
    fn add(&mut self, boards: Vec<Board>);
    fn next(&mut self) -> Option<Board>;
    fn len(&self) -> usize;
    fn empty(&self) -> bool;
}
pub trait FilterQueue: BoardQueue {
    fn filter(&mut self, predicate: impl Fn(&Board) -> bool) -> Vec<Board>;
}
pub struct BoardQueueImpl {
    underlying_structure: PriorityQueue<Board, usize>,
}
impl BoardQueueImpl {
    pub fn new(starting_board: Board) -> BoardQueueImpl {
        let mut queue = Self {
            underlying_structure: PriorityQueue::new(),
        };
        queue.add_single(starting_board);
        queue
    }
}
impl BoardQueue for BoardQueueImpl {
    fn add_single(&mut self, board: Board) {
        self.underlying_structure
            .push(board.clone(), 200 - board.theoretical_minimum());
    }
    fn add(&mut self, boards: Vec<Board>) {
        for board in boards {
            self.add_single(board);
        }
    }
    fn next(&mut self) -> Option<Board> {
        self.underlying_structure
            .pop()
            .map(|(board, _priority)| board)
    }
    fn len(&self) -> usize {
        self.underlying_structure.len()
    }
    fn empty(&self) -> bool {
        self.underlying_structure.is_empty()
    }
}
