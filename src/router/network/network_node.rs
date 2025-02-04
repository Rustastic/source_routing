
use wg_2024::packet::NodeType;

use wg_2024::network::NodeId;

use std::cell::RefCell;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct NetworkNode {
    pub(crate) neighbours: RefCell<Vec<NodeId>>,
    pub(crate) node_type: NodeType,
}

impl NetworkNode {
    //constructor
    pub(crate) fn new(node_type: NodeType) -> Self {
        Self {
            neighbours: RefCell::new(Vec::new()),
            node_type,
        }
    }
}

impl NetworkNode {
    //methods
    /// # Note
    /// Does not check if the id is valid, so you have to ensure that the id is already in the network
    pub(crate) fn add_neighbour(&self, id: NodeId) {
        self.neighbours.borrow_mut().push(id);
    }
    /// Add some ids to the neightbours calling `std::vec::reserve()` before
    /// # Note
    /// Does not check if the ids are valid, so you have to ensure that the ids are already in the network
    pub(crate) fn bulk_add_neighbours(&self, ids: Vec<NodeId>) {
        self.neighbours.borrow_mut().reserve(ids.len());
        for id in ids {
            self.add_neighbour(id);
        }
    }
    /// # Note
    /// Does not preserve order in the vector
    pub(crate) fn remove_neighbour(&self, id: NodeId) {
        if let Some(index) = self.neighbours.borrow().iter().position(|&i| i == id) {
            self.neighbours.borrow_mut().swap_remove(index);
        }
    }
}
