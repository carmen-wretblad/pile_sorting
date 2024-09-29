use crate::board::Board;

pub trait BoardQueue {
    fn add(&mut self, board: Board);
    fn next(&mut self) -> Option<Board>;
    fn len(&self) -> usize;
    fn empty(&self) -> bool;
}
pub trait FilterQueue: BoardQueue {
    fn filter(&mut self, predicate: impl Fn(&Board) -> bool) -> Vec<Board>;
}
pub struct BoardQueueImpl {}
impl BoardQueue for BoardQueueImpl {
    fn add(&mut self, board: Board) {
        unimplemented!()
    }
    fn next(&mut self) -> Option<Board> {
        unimplemented!()
    }
    fn len(&self) -> usize {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
}
