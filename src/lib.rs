#![allow(dead_code)]

pub type Move = [usize; 2];
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub mod board;
pub mod config;
pub mod program;
pub mod vector_util;
