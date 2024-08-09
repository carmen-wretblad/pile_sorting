use ::sorting::board::Board;
use ::sorting::program::BFS;
//40 000 boards fine, 200 000 slowdown
fn main() {
    let vec = vec![1, 6, 12, 2, 9, 5, 4, 3, 10, 7, 8, 11];
    let board1 = Board::new(&vec, 4);
    let mut bfs1 = BFS::new(&board1);
    let mut counter1 = 0;
    while !bfs1.internal_step() {
        //counter1 += 1;
        //println!("{}", counter1);
    }
    println!("done! {}", counter1);
}
