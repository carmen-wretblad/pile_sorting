use ::sorting::board::Board;
use ::sorting::program::BFS;
fn main() {
    run_board();
}
fn run_board() {
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

fn stats() {}
