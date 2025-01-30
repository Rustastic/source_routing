use std::{collections::HashMap, path};

use wg_2024::{network::NodeId, packet::NodeType};

use crate::error::{
    Result,
    RouterError::{IdAlreadyPresent, IdNotFound, RemoveSelfErr},
};

#[derive(Debug)]
pub struct Network {
    root: NodeId,
    network: HashMap<NodeId, NetworkNode>,
}

impl Network {
    pub fn new(root: NodeId, root_type: NodeType) -> Self {
        let mut network = HashMap::new();
        network.insert(root, NetworkNode::new(root_type));
        Self { root, network }
    }
    /// # Returns:
    /// - `Err(IdNotFound)`
    pub fn get(&self, id: NodeId) -> Result<&NetworkNode> {
        self.network.get(&id).ok_or(IdNotFound { id })
    }
    /// # Returns:
    /// - `Err(IdNotFound)`
    pub fn get_mut(&mut self, id: NodeId) -> Result<&mut NetworkNode> {
        self.network.get_mut(&id).ok_or(IdNotFound { id })
    }
    pub fn update_from_path_trace(&mut self, path_trace: &[(NodeId, NodeType)]) {
        for i in 0..path_trace.len() - 1 {
            let (id1, type1) = path_trace[i];
            let (id2, type2) = path_trace[i + 1];
            if (!self.contains_id(id1)) {
                self.add_empty_node(id1, type1);
            }
            if (!self.contains_id(id2)) {
                self.add_empty_node(id2, type2);
            }
            self.add_link(id1, id2).inspect_err(|_e|todo!("send to sim controller"));
        }
    }
    pub fn contains_id(&self, key: NodeId) -> bool {
        self.network.contains_key(&key)
    }
    /// Add a node without neighbours to the network
    fn add_empty_node(&mut self, id: NodeId, node_type: NodeType) -> Result<()> {
        if self.network.contains_key(&id) {
            return Err(IdAlreadyPresent { id, node_type });
        }
        self.network.insert(id, NetworkNode::new(node_type));
        Ok(())
    }
    /// Add `(id1, id2)` and `(id2, id1)` because link are undirected
    /// # Returns
    /// - `Err(IdNotFound)` if one of ithe ids is not in the network
    fn add_link(&mut self, id1: NodeId, id2: NodeId) -> Result<()> {
        if !self.contains_id(id2) {
            return Err(IdNotFound { id: id2 });
        }
        self.network
            .get_mut(&id1)
            .ok_or(IdNotFound { id: id1 })?
            .add_neighbour(id2);
        self.network
            .get_mut(&id2)
            .ok_or(IdNotFound { id: id2 })?
            .add_neighbour(id1);
        Ok(())
    }
    /// Remove the node specified from the network
    /// # Returns
    /// - `Err(RemoveSelfErr)` if the id is the root
    /// - `Err(IdNotFound)`
    /// - `Some(id)` id the node deleted
    fn remove_node(&mut self, id: NodeId) -> Result<NodeId> {
        if self.root == id {
            return Err(RemoveSelfErr);
        }
        self.network.remove(&id).ok_or(IdNotFound { id })?;
        for v in self.network.values_mut() {
            v.remove_neighbour(id);
        }
        Ok(id)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct NetworkNode {
    neighbours: Vec<NodeId>,
    node_type: NodeType,
}

impl NetworkNode {
    fn new(node_type: NodeType) -> Self {
        Self {
            neighbours: Vec::new(),
            node_type,
        }
    }
    /// # Note
    /// Does not check if the id is valid, so you have to ensure that the id is already in the network
    fn add_neighbour(&mut self, id: NodeId) {
        self.neighbours.push(id);
    }
    /// Add some ids to the neightbours calling `std::vec::reserve()` before
    /// # Note
    /// Does not check if the ids are valid, so you have to ensure that the ids are already in the network
    fn bulk_add_neighbours(&mut self, ids: Vec<NodeId>) {
        self.neighbours.reserve(ids.len());
        for id in ids {
            self.add_neighbour(id);
        }
    }
    /// # Note
    /// Does not preserve order in the vector
    fn remove_neighbour(&mut self, id: NodeId) {
        if let Some(index) = self.neighbours.iter().position(|&i| i == id) {
            self.neighbours.swap_remove(index);
        }
    }
}
