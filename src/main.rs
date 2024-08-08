use ::sorting::board::Board;
use ::sorting::program::BFS;
fn main() {
    let vec = vec![3, 5, 4, 2, 1, 7, 8, 9, 10];
    let board1 = Board::new(&vec, 4);
    let board2 = Board::new(&vec, 7);
    let mut bfs1 = BFS::new(&board1);
    let mut bfs2 = BFS::new(&board2);
    let mut counter1 = 0;
    let mut counter2 = 0;
    while bfs1.internal_step() == false {
        counter1 += 1;
    }
    while bfs2.internal_step() == false {
        counter2 += 1;
    }
    println!("counter1: {counter1}");
    println!("counter:2 {counter2}");
}
