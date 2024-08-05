#![allow(unused)]
// ### Questions ###
// Is there a trait for structs that can be created with New?
// Answered: What happens when a struct is moved: Depends
// If a struct has a field, is the field moved "with it?"
// -------------------------------------------------------
// Instresting perspecitve: you can see this as a single state machine and you want to find the
// smallest amount of signals to get it to move from one state to another
// ### TODO ###
// Implement "Solution"
use crate::{board::*, Move};
pub trait Program: Iterator {
    fn starting_state(&self) -> &Board;
    fn done(&self) -> bool;
    /// Runs the program until a new Move has been reached, must change result of done method when
    /// applicable
    fn step(&mut self) -> Option<Move>;
    /// Runs the program to completion.
    /// Will return new moves made only
    fn run(&mut self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::<Move>::new();
        while (!self.done()) {
            if let Some(value) = self.step() {
                vec.push(value)
            }
        }
        vec
    }
    /// Returns all moves so far
    fn progress(&self) -> &Vec<Move>;
}
/// stuff all programs should contain
pub struct BFS {
    name: String,
    starting_board: Board,
    moves: Vec<Move>,
    done_flag: bool,
}
