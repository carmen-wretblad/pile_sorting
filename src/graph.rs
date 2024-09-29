use crate::BoardRep;
use crate::RelMove;
use crate::RelSolution;

pub trait GraphID {
    fn id(self) -> BoardRep;
}

trait Graph {
    fn contains(&self) -> bool;
    fn add(&mut self, parent: &BoardRep, children: BoardRep);
}

trait GraphInfo: Graph {
    fn nbr_nodes(&self) -> usize;
    fn nbr_edges(&self) -> usize;
}

trait SolvableGraph: Graph {
    fn solved(&self) -> bool;
    fn solution(&self) -> Option<RelSolution>;
    /// Not required to be exhaustive
    fn solutions(&self) -> Option<Vec<RelSolution>>;
}
trait DebugGraph: Graph {
    fn all_solutions(&self) -> Option<Vec<RelSolution>>;
    fn shortest_path(&self) -> Option<Vec<RelSolution>>;
    fn longest_path(&self) -> Option<Vec<RelSolution>>;
}
pub struct GraphImpl {}
impl Graph for GraphImpl {
    fn contains(&self) -> bool {
        unimplemented!();
    }
    fn add(&mut self, parent: &BoardRep, children: BoardRep) {
        unimplemented!();
    }
}
