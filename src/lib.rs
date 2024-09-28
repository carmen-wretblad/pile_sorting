#[allow(unused, dead_code)]
pub type BoardRep = Vec<u8>;
pub type AbsMove = [usize; 2];
pub type RelMove = [usize; 2];
pub mod bfs;
pub mod board;
pub mod config;
pub mod history_tracker;
pub mod node_content;
pub mod node_holder;
pub mod sortedness;
pub mod translator;
pub mod validator;
pub mod vector_util;
