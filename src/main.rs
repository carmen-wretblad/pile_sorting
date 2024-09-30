#[allow(unused, dead_code)]
use ::sorting::bfs::BFS;
use ::sorting::board::Board;

use sorting::node_holder::NodeHolder;
use sorting::vector_util::{self, all_sequences};
use sorting::{bfs, board, graph_queue_solver};
fn main() {
    //stats();
    //test_node_holder();
    //test_node_holder_expensive();
    //run_board();
    test_graph_solver();
}
fn test_graph_solver() {
    //let vec = [1, 8, 4, 7, 2, 3, 9, 6, 5];
    //let board = Board::new(&vec, 5);
    for i in 5..100 {
        let pile = vector_util::random_vec(i);
        let board = Board::new(&pile, 4);
        println!("pile_height tested {i}");
        let mut graph_solver = graph_queue_solver::GraphQueueSolverImpl::new(board);
        graph_solver.test();
    }
    //let mut graph_solver = graph_queue_solver::GraphQueueSolverImpl::new(board);
    //graph_solver.test();
}
fn run_board() {
    let vec = [1, 8, 4, 10, 7, 2, 3, 9, 6, 5];
    let board = Board::new(&vec, 4);
    let mut bfs = BFS::new(&board, sorting::bfs::MoveChoice::Good);
    println!("{}", bfs.solve().unwrap().len());
    println!("bfs board counter {} ", bfs.board_counter);
    println!("bfs steps: {}", bfs.step_counter);
}

fn test_node_holder() {
    let vec = [1, 9, 5, 6, 3, 8, 4, 7, 12, 2, 11, 14, 13, 10];
    let board = Board::new(&vec, 5);
    let mut holder = NodeHolder::new(&board);
    while !holder.is_solved() {
        holder.step();
    }
    println!("holder boards found {}", holder.board_counter);
    println!("holder steps {}", holder.steps);
    println!("holder bad boards {}", holder.bad_boards.len());
}
fn test_node_holder_expensive() {
    let vec = [
        1, 9, 18, 5, 6, 3, 8, 15, 4, 17, 7, 12, 2, 11, 16, 14, 13, 10,
    ];
    let board = Board::new(&vec, 5);
    let mut holder = NodeHolder::new(&board);
    while !holder.is_solved() {
        holder.step();
    }
    println!("holder boards found {}", holder.board_counter);
    println!("holder steps {}", holder.steps);
    println!("holder bad boards {}", holder.bad_boards.len());
}
fn stats() {
    for nbr_cards in 5..8 {
        for nbr_piles in 4..6 {
            let mut longest = 0;
            let mut average = 0;
            let mut amount_looked_at = 0;
            for sequence in all_sequences(nbr_cards) {
                let board = Board::new(&sequence, nbr_piles);
                let lenght = BFS::new(&board, bfs::MoveChoice::Good)
                    .solve()
                    .expect("must have a solution")
                    .len();
                average += lenght;
                amount_looked_at += 1;
                if lenght > longest {
                    longest = lenght;
                }
            }
            let average = average / amount_looked_at;

            println!(
                "cards: {}, piles {}, max: {}, average {}",
                nbr_cards, nbr_piles, longest, average
            );
        }
    }
}
