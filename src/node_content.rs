use crate::BoardRep;
use crate::RelMove;
#[derive(PartialEq, Eq, Clone)]
enum NodeStatus {
    Building,
    Done,
    Empty,
}

pub type NodeRelation = (BoardRep, RelMove);

#[derive(Clone)]
pub struct NodeContent {
    pub parents: NodeRelations,
    pub children: NodeRelations,
}
impl NodeContent {
    pub fn new() -> NodeContent {
        Self {
            parents: NodeRelations::new(),
            children: NodeRelations::new(),
        }
    }
}

#[derive(Clone)]
pub struct NodeRelations {
    status: NodeStatus,
    items: Vec<NodeRelation>,
}
impl NodeRelations {
    pub fn get_items(&self) -> Vec<NodeRelation> {
        self.items.clone()
    }
    pub fn new() -> Self {
        Self {
            status: NodeStatus::Building,
            items: Vec::new(),
        }
    }
    pub fn add_item(&mut self, parent: &NodeRelation) {
        assert!(self.status == NodeStatus::Building);
        self.items.push(parent.clone());
    }

    pub fn has_item(&self, item_representation: &BoardRep) -> bool {
        self.items.iter().any(|x| x.0 == *item_representation)
    }
    pub fn remove_item(&mut self, item_representation: &BoardRep) {
        assert!(self.is_done());
        assert!(self.has_item(item_representation));
        self.items.retain(|x| x.0 == *item_representation);
        if self.items.is_empty() {
            self.status = NodeStatus::Empty
        }
    }
    pub fn set_done(&mut self) {
        assert!(self.is_building());
        match self.items.is_empty() {
            true => self.status = NodeStatus::Empty,
            false => self.status = NodeStatus::Done,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.status == NodeStatus::Empty
    }
    pub fn is_building(&self) -> bool {
        self.status == NodeStatus::Building
    }
    pub fn is_done(&self) -> bool {
        self.status == NodeStatus::Done
    }
}
