use std::{cell::RefCell, collections::{HashMap, HashSet, VecDeque}}
;
use wg_2024::{network::NodeId, packet::NodeType};
use crate::error::{
    Result,
    RouterError::{IdAlreadyPresent, IdNotFound, ParentsMalformed, RemoveSelfErr, RouteNotFound},
};

pub type Path = Vec<NodeId>;

#[cfg(test)]
mod test;

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
    pub fn contains_id(&self, key: NodeId) -> bool {
        self.network.contains_key(&key)
    }
    pub fn update_from_path_trace(&mut self, path_trace: &[(NodeId, NodeType)]) {
        for i in 0..path_trace.len() - 1 {
            let (id1, type1) = path_trace[i];
            let (id2, type2) = path_trace[i + 1];
            if !self.contains_id(id1) {
                let _ = self.add_empty_node(id1, type1);
            }
            if !self.contains_id(id2) {
                let _ = self.add_empty_node(id2, type2);
            }
            let _ = self.add_link(id1, id2);
        }
    }
    pub fn get_routes(&self, destination: NodeId) -> Result<Path> {
        let parents = self.bfs().or(Err(RouteNotFound { destination }))?;
        let path = parents_to_path(&parents, destination)?;
        Ok(path)
    }
    /// Remove every neighbour that is no longer in the network
    fn try_fix_network(&mut self) {
        for node in self.network.values(){
            for &id in node.neighbours.borrow().iter() {
                if !self.contains_id(id) {
                    node.remove_neighbour(id);
                }
            }
        }
    }
    /// Compute vector of parent of the network starting from the root
    /// # Returns
    /// - `Ok(HashMap<u,v>)` : `v` is the father of `u`
    /// - `Err(IdNotFound)` : if the network refer to a node no longer in the network
    fn bfs(&self) -> Result<HashMap<NodeId, Option<NodeId>>> {
        let mut queue = VecDeque::new();
        queue.push_back(self.root);

        let mut visited = HashSet::new();
        visited.insert(self.root);

        let mut parents = HashMap::new();
        parents.insert(self.root, None);

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap_or_else(|| unreachable!());
            for &v in self.get(u)?.neighbours.borrow().iter() {
                if !visited.contains(&v) {
                    parents.insert(v, Some(u));
                    visited.insert(v);
                    queue.push_back(v);
                }
            }
        }
        Ok(parents)
    }
    /// Add a node without neighbours to the network
    /// # Returns
    /// - `Err(IdAlreadyPresent)` if the id is already in the network
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
    /// - `Ok(id)` id the node deleted
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
/// Returns a path from the vector of parents
/// # Returns
/// - `Err(ParentsMalformed)` if the vector of parents is malformed
fn parents_to_path(
    parents: &HashMap<NodeId, Option<NodeId>>,
    destination: NodeId,
) -> Result<Path> {
    let mut path = vec![destination];
    let mut current = &destination;
    while let Some(parent) = parents.get(current).ok_or(ParentsMalformed {
        parents: parents.clone(),
        destination,
    })? {
        path.push(*parent);
        current = parent;
    }
    path.reverse();
    Ok(path)
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct NetworkNode {
    neighbours: RefCell<Vec<NodeId>>,
    node_type: NodeType,
}

impl NetworkNode {
    fn new(node_type: NodeType) -> Self {
        Self {
            neighbours: RefCell::new(Vec::new()),
            node_type,
        }
    }
    /// # Note
    /// Does not check if the id is valid, so you have to ensure that the id is already in the network
    fn add_neighbour(&self, id: NodeId) {
        self.neighbours.borrow_mut().push(id);
    }
    /// Add some ids to the neightbours calling `std::vec::reserve()` before
    /// # Note
    /// Does not check if the ids are valid, so you have to ensure that the ids are already in the network
    fn bulk_add_neighbours(&self, ids: Vec<NodeId>) {
        self.neighbours.borrow_mut().reserve(ids.len());
        for id in ids {
            self.add_neighbour(id);
        }
    }
    /// # Note
    /// Does not preserve order in the vector
    fn remove_neighbour(&self, id: NodeId) {
        // if let Some(index) = self.neighbours.iter().position(|&i| i == id) {
        //     self.neighbours.swap_remove(index);
        // }
        if let Some(index) = self.neighbours.borrow().iter().position(|&i| i == id) {
            self.neighbours.borrow_mut().swap_remove(index);
        }
    }
}
