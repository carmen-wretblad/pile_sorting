//#![allow(dead_code)]

pub type AbsMove = [usize; 2];
pub type RelMove = [usize; 2];
pub mod bfs;
pub mod board;
pub mod board_rep;
pub mod config;
pub mod node_holder;
pub mod validator;
pub mod vector_util;
