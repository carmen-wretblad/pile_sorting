use crate::BoardRep;
use crate::RelMove;
use crate::RelSolution;

trait GraphID {
    fn ID(self) -> [BoardRep];
}

trait Graph {
    fn contains(&self) -> bool;
    fn add(&mut self, parent: &BoardRep, children: BoardRep);
}

trait GraphInfo {
    fn nbr_nodes(&self) -> usize;
    fn nbr_edges(&self) -> usize;
}

trait SolvableGraph {
    fn solved(&self) -> bool;
    fn solution(&self) -> Option<RelSolution>;
    /// Not required to be exhaustive
    fn solutions(&self) -> Option<Vec<RelSolution>>;
}
trait DebugGraph {
    fn all_solutions(&self) -> Option<Vec<RelSolution>>;
    fn shortest_path(&self) -> Option<Vec<RelSolution>>;
    fn longest_path(&self) -> Option<Vec<RelSolution>>;
}
