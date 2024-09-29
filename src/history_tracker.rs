use crate::translator::Translator;
use crate::AbsMove;
use std::usize;

// IDEA: use a singular vec and translate the indicies instead

const BLOCKED: bool = true;
const USABLE: bool = false;

#[derive(Debug, Clone)]
pub struct HistoryTrackerImpl {
    nbr_piles: usize,
    last_move: Option<AbsMove>,
    blocker_matrix: Vec<Vec<bool>>,
}

pub trait Reverter {
    fn revert(&self, piles: &Vec<&[u8]>, translator: Translator);
    fn last_move(&self) -> Option<AbsMove>;
}

pub trait HistoryTracker {
    fn unnecessary(&self, move_command: AbsMove) -> bool;
    fn remove_unnecessary(&self, move_commands: Vec<AbsMove>) -> Vec<AbsMove>;
    fn last_move(&self) -> Option<AbsMove>;
    fn update(&mut self, move_command: AbsMove);
}
trait BlockerMatrixUtil {
    fn get_from_to(&self, from: usize, to: usize) -> bool;
    fn set_from_to(&mut self, from: usize, to: usize, status: bool);
    fn set_from(&mut self, from: usize, status: bool);
    fn set_to(&mut self, to: usize, status: bool);
}

impl HistoryTrackerImpl {
    pub fn new(nbr_piles: usize) -> Self {
        let mut matrix = Vec::new();
        for i in 0..nbr_piles {
            matrix.push(Vec::new());
            for _ in 0..nbr_piles {
                matrix[i].push(USABLE);
            }
        }
        Self {
            nbr_piles,
            last_move: None,
            blocker_matrix: matrix,
        }
    }
}
impl HistoryTracker for HistoryTrackerImpl {
    fn unnecessary(&self, move_command: AbsMove) -> bool {
        self.get_from_to(move_command[0], move_command[1])
    }
    fn remove_unnecessary(&self, move_commands: Vec<AbsMove>) -> Vec<AbsMove> {
        move_commands
            .into_iter()
            .filter(|x| !self.unnecessary(*x))
            .collect()
    }
    fn update(&mut self, move_command: AbsMove) {
        self.set_to(move_command[0], USABLE);
        self.set_from(move_command[1], BLOCKED);
        self.last_move = Some(move_command);
    }
    fn last_move(&self) -> Option<AbsMove> {
        self.last_move
    }
}
/*impl Reverter for HistoryTrackerImpl {
fn revert(&self, piles: &Vec<&[u8]>, translator: Translator) {
    unimplemented!();
    match self.last_move() {
        None => panic!(),
        Some(some_move) => {
            let mut the_move = some_move;
            the_move.reverse();
            let mut board = self.clone();
            if board.last_shrunk {
                board.piles[self.pos_of_highest_card]
                    .insert(0, u8::try_from(self.nbr_cards + 1).unwrap());
            }
            board.perform_move(self.translator.into_rel_move(the_move));
            board
        }
    }
}*/

impl BlockerMatrixUtil for HistoryTrackerImpl {
    fn get_from_to(&self, from: usize, to: usize) -> bool {
        self.blocker_matrix[from][to]
    }
    fn set_from_to(&mut self, from: usize, to: usize, status: bool) {
        self.blocker_matrix[from][to] = status;
    }
    fn set_from(&mut self, from: usize, status: bool) {
        for i in 0..self.nbr_piles {
            self.blocker_matrix[from][i] = status;
        }
    }
    fn set_to(&mut self, to: usize, status: bool) {
        for i in 0..self.nbr_piles {
            self.blocker_matrix[i][to] = status;
        }
    }
}
