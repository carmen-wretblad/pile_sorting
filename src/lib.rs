#![allow(unused, dead_code)]
// --- Hardcoded ---
pub const MAX_NBR_OF_CARDS: usize = 100;
pub const MIN_NBR_OF_CARDS: usize = 3;
pub const SOLUTION_PILE: [u8; 2] = [2, 1];
// --- Config ---
pub const VALIDATOR_SHOULD_PRINT: bool = true;
pub const BFS_SHOULD_PRINT_FOUND_BOARDS: bool = false;
pub const BFS_SHOULD_PRINT_STEP_COUNTER: bool = false;
pub const NBR_PILES: usize = 5;
// --- Type Aliases ---
pub type BoardRep = Vec<u8>;
pub type AbsMove = [usize; 2];
pub type RelMove = [usize; 2];
pub type RelSolution = Vec<RelMove>;
pub type AbsSolution = Vec<AbsMove>;
// --- Modules ---
pub mod bfs;
pub mod board;
pub mod board_queue;
pub mod graph;
pub mod graph_queue_solver;
pub mod history_tracker;
pub mod node_content;
pub mod node_holder;
pub mod sortedness;
pub mod translator;
pub mod validator;
pub mod vector_util;
