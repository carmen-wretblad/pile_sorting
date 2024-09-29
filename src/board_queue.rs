use crate::board::Board;

trait BoardQueue {
    fn add(&mut self, board: Board);
    fn next(&mut self) -> Option<Board>;
    fn len(&self) -> usize;
    fn empty(&self) -> bool;
}
trait FilterQueue {
    fn filter(&mut self, predicate: impl Fn(&Board) -> bool) -> Vec<Board>;
}
