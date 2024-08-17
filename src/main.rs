use ::sorting::board::Board;
use ::sorting::program::BFS;

//40 000 boards fine, 200 000 slowdown
fn main() {
    //run_board_1();
    run_board_2();
}
fn run_board_1() {
    let vec1 = vec![2, 5, 3, 4, 6, 1, 7];
    let board1 = Board::new(&vec1, 4);
    let mut bfs1 = BFS::new(&board1, sorting::program::MoveChoice::Good);
    while !bfs1.internal_step() {}
    println!(" Done, checking solution");
    match bfs1.get_full_solution() {
        Some(_) => println!("success!"),
        None => println!("failure"),
    }
}
fn run_board_2() {
    let vec2 = vec![1, 5, 2, 4, 3];
    let board2 = Board::new(&vec2, 4);
    let mut bfs2 = BFS::new(&board2, sorting::program::MoveChoice::Valid);
    bfs2.solve();
}
