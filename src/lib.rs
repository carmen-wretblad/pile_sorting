#[allow(unused, dead_code)]
pub type BoardRep = Vec<u8>;
pub type AbsMove = [usize; 2];
pub type RelMove = [usize; 2];
pub type RelSolution = Vec<RelMove>;
pub type AbsSolution = Vec<AbsMove>;
pub mod bfs;
pub mod board;
pub mod board_queue;
pub mod config;
pub mod graph;
pub mod graph_queue_solver;
pub mod history_tracker;
pub mod node_content;
pub mod node_holder;
pub mod sortedness;
pub mod translator;
pub mod validator;
pub mod vector_util;
