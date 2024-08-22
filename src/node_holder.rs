use crate::board::Board;
use crate::BoardRep;
use crate::RelMove;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::usize;

#[derive(PartialEq, Eq, Clone)]
enum NodeStatus {
    Building,
    Done,
    Empty,
}

type NodeRelation = (BoardRep, RelMove);

#[derive(Clone)]
struct NodeRelations {
    status: NodeStatus,
    parents: Vec<NodeRelation>,
}
impl NodeRelations {
    fn new_empty() -> Self {
        Self {
            status: NodeStatus::Building,
            parents: Vec::new(),
        }
    }
    fn add_parent(&mut self, parent: &NodeRelation) {
        assert!(self.status == NodeStatus::Building);
        self.parents.push(parent.clone());
    }
    fn has_parent(&self, parent_representation: &BoardRep) -> bool {
        self.parents.iter().any(|x| x.0 == *parent_representation)
    }
    fn remove_parent(&mut self, parent_representation: &BoardRep) {
        assert!(self.is_done());
        assert!(self.has_parent(parent_representation));
        self.parents.retain(|x| x.0 == *parent_representation);
        if self.parents.is_empty() {
            self.status = NodeStatus::Empty
        }
    }
    fn status(&self) -> NodeStatus {
        self.status.clone()
    }
    fn set_done(&mut self) {
        assert!(self.is_building());
        match self.parents.is_empty() {
            true => self.status = NodeStatus::Empty,
            false => self.status = NodeStatus::Done,
        }
    }
    fn is_empty(&self) -> bool {
        self.status == NodeStatus::Empty
    }
    fn is_building(&self) -> bool {
        self.status == NodeStatus::Building
    }
    fn is_done(&self) -> bool {
        self.status == NodeStatus::Done
    }
}
struct Children {
    children: Option<Vec<(BoardRep, RelMove)>>,
}
#[derive(Clone)]
struct Node {
    key: BoardRep,
    parents: NodeRelations,
    children: NodeRelations,
    nbr_cards: usize,
}
impl Node {
    fn new(board: &Board) -> Self {
        unimplemented!();
    }
}
impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

struct ProgramInfo {
    starting_board: Board,
    end_board: Board,
    nbr_piles: usize,
    innitial_nbr_cards: usize,
}

struct NodeHolder {
    info: ProgramInfo,
    steps_taken: usize,
    solved_flag: bool,
    new_generation_board: NewGenerationHolder,
    nodes: HashSet<Node>,
}
impl NodeHolder {
    fn new(board: &Board) -> Self {
        let mut nodes: HashSet<Node> = HashSet::new();
        let start_node: Node = Node::new(board);
        nodes.insert(start_node);
        NodeHolder {
            info: ProgramInfo {
                starting_board: board.clone(),
                end_board: Board::new_solved_board(board.piles.len()),
                nbr_piles: board.piles.len(),
                innitial_nbr_cards: board.nbr_cards,
            },
            steps_taken: 0,
            solved_flag: false,
            new_generation_board: NewGenerationHolder {
                generation_status: GenerationStatus::Standard,
                new_generation: vec![board.clone()],
            },
            nodes,
        }
    }

    fn step(&self) {
        self.new_generation();
        self.update_global_heuristic();
        self.remove_childless();
        self.apply_global_heuristic();
        self.remove_unneeded();
    }
    fn new_generation(&self) {
        self.spawn_new_generation();
        self.is_solved();
        self.update_local_heuristic();
        self.prune_new_generation();
        self.generation_shift();
    }
    fn update_global_heuristic(&self) {
        unimplemented!();
    }
    fn update_local_heuristic(&self) {
        unimplemented!();
    }
    fn remove_childless(&self) {
        unimplemented!();
    }
    fn remove_unneeded(&self) {
        unimplemented!();
    }
    fn spawn_new_generation(&self) {
        unimplemented!();
    }
    fn is_solved(&self) {
        unimplemented!();
    }
    fn prune_new_generation(&self) {
        unimplemented!();
    }
    fn generation_shift(&self) {
        unimplemented!();
    }
    fn apply_global_heuristic(&self) {
        unimplemented!();
    }
}

struct NewGenerationHolder {
    generation_status: GenerationStatus,
    new_generation: Vec<Board>,
}
impl NewGenerationHolder {
    fn generation_shift(&mut self) {
        unimplemented!();
    }
}
enum GenerationStatus {
    New,
    NewlySpawned,
    Pruned,
    Standard,
    Used,
}
