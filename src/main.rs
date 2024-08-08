use ::sorting::board::Board;
use ::sorting::program::BFS;
//40 000 boards fine, 200 000 slowdown
fn main() {
    let vec = vec![3, 5, 4, 2, 1, 11, 7, 8, 9, 6, 10];
    let board1 = Board::new(&vec, 7);
    let mut bfs1 = BFS::new(&board1);
    let mut counter1 = 0;
    while bfs1.internal_step() == false {
        counter1 += 1;
    }
}
