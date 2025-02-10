use crate::error::{
    Result,
    RouterError::{IdAlreadyPresent, IdNotFound, ParentsMalformed, RemoveSelfErr, RouteNotFound},
};
use log::info;
use network_node::NetworkNode;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
};
use wg_2024::{network::NodeId, packet::NodeType};

pub type Path = Vec<NodeId>;

mod network_node;
#[cfg(test)]
mod test;

#[allow(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Network {
    root: NodeId,
    network: HashMap<NodeId, NetworkNode>,
    server_list: HashSet<NodeId>,
}

impl Network {
    //constructor
    pub fn new(root: NodeId, root_type: NodeType) -> Self {
        let mut network = HashMap::new();
        network.insert(root, NetworkNode::new(root_type));
        Self {
            root,
            network,
            server_list: HashSet::new(),
        }
    }
}

impl Network {
    //methods
    pub fn contains_id(&self, key: NodeId) -> bool {
        self.network.contains_key(&key)
    }
    pub fn update_from_path_trace(&mut self, path_trace: &[(NodeId, NodeType)]) {
        if let Some((first_id, first_type)) = path_trace.first() {
            if !self.contains_id(*first_id) {
                let _ = self.add_empty_node(*first_id, *first_type);
            }
            let _ = self.add_link(self.root, *first_id);
        }
        for i in 0..path_trace.len() - 1 {
            let (id1, type1) = path_trace[i];
            if let NodeType::Server = type1 {
                self.server_list.insert(id1);
            }
            let (id2, type2) = path_trace[i + 1];
            if let NodeType::Server = type2 {
                self.server_list.insert(id2);
            }
            if !self.contains_id(id1) {
                let _ = self.add_empty_node(id1, type1);
            }
            if !self.contains_id(id2) {
                let _ = self.add_empty_node(id2, type2);
            }
            let _ = self.add_link(id1, id2);
        }
    }
    /// Remove every neighbour that is no longer in the network
    pub fn try_fix_network(&mut self) {
        for node in self.network.values() {
            for &id in node.neighbours.borrow().iter() {
                if !self.contains_id(id) {
                    node.remove_neighbour(id);
                }
            }
        }
    }
    /// Remove the node specified from the network
    /// # Errors
    /// - `Err(RemoveSelfErr)` if the id is the root
    /// - `Err(IdNotFound)`
    /// - `Ok(id)` id the node deleted
    pub fn remove_node(&mut self, id: NodeId) -> Result<NodeId> {
        if self.root == id {
            return Err(Box::new(RemoveSelfErr));
        }
        if self.get(id)?.node_type == NodeType::Server {
            self.server_list.remove(&id);
        }

        self.network.remove(&id).ok_or(IdNotFound(id))?;
        for v in self.network.values_mut() {
            v.remove_neighbour(id);
        }
        Ok(id)
    }
    /// # Errors
    /// - `IdAlreadyPresent`
    pub fn add_neighbour(&mut self, id: NodeId) -> Result<()> {
        self.add_empty_node(id, NodeType::Drone)?;
        let _ = self.add_link(id, self.root);
        Ok(())
    }
    /// Compute vector of parent of the network starting from the root
    /// # Errors
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
    /// # Errors
    /// - `Err(IdAlreadyPresent)` if the id is already in the network
    fn add_empty_node(&mut self, id: NodeId, node_type: NodeType) -> Result<()> {
        if self.network.contains_key(&id) {
            return Err(Box::new(IdAlreadyPresent { id, node_type }));
        }
        if node_type == NodeType::Server {
            self.server_list.insert(id);
        }
        self.network.insert(id, NetworkNode::new(node_type));
        Ok(())
    }
    /// Add `(id1, id2)` and `(id2, id1)` because link are undirected
    /// # Errors
    /// - `Err(IdNotFound)` if one of ithe ids is not in the network
    fn add_link(&mut self, id1: NodeId, id2: NodeId) -> Result<()> {
        if !self.contains_id(id2) {
            return Err(Box::new(IdNotFound(id2)));
        }
        self.network
            .get_mut(&id1)
            .ok_or(IdNotFound(id1))?
            .add_neighbour(id2);
        self.network
            .get_mut(&id2)
            .ok_or(IdNotFound(id2))?
            .add_neighbour(id1);
        Ok(())
    }
    /// Compute all the paths between the root and a destination
    ///
    fn dfs(
        &self,
        current: u8,
        destination: u8,
        visited: &mut HashSet<u8>,
        current_path: &mut Vec<u8>,
        paths: &mut Vec<Vec<u8>>,
    ) {
        if current == destination {
            paths.push(current_path.clone());
            return;
        }

        visited.insert(current);

        if let Some(current_node) = self.network.get(&current) {
            for &neighbor in current_node.neighbours.borrow().iter() {
                if visited.contains(&neighbor) {
                    continue;
                }

                let Some(neighbor_node) = self.network.get(&neighbor) else { continue };

                let should_visit = match neighbor_node.node_type {
                    NodeType::Drone => true,
                    _ => neighbor == destination,
                };

                if should_visit {
                    current_path.push(neighbor);
                    self.dfs(neighbor, destination, visited, current_path, paths);
                    current_path.pop();
                }
            }
        }

        visited.remove(&current);
    }
}

impl Network {
    //getter/setter
    /// # Errors:
    /// - `Err(IdNotFound)`
    pub fn get(&self, id: NodeId) -> Result<&NetworkNode> {
        self.network.get(&id).ok_or(Box::new(IdNotFound(id)))
    }
    /// # Errors:
    /// - `Err(IdNotFound)`
    pub fn get_mut(&mut self, id: NodeId) -> Result<&mut NetworkNode> {
        self.network.get_mut(&id).ok_or(Box::new(IdNotFound(id)))
    }
    /// # Errors
    /// - `Err(RouteNotFound)` if the destionation is unreachable
    pub fn get_routes(&self, destination: NodeId) -> Result<Path> {
        let parents = self.bfs().or(Err(RouteNotFound { destination }))?;
        let path = parents_to_path(&parents, destination)?;
        Ok(path)
    }
    pub fn get_server_list(&self) -> HashSet<NodeId> {
        self.server_list.clone()
    }
    /// Useful when a drone in a path has a high pdr,
    /// the client/server can call this method and decide which path
    /// the dropped packet will be sent through.
    pub fn multiple_paths(&self, destination_id: NodeId) -> Vec<Path> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut current_path = vec![self.root];
        self.dfs(
            self.root,
            destination_id,
            &mut visited,
            &mut current_path,
            &mut paths,
        );
        paths
    }
    pub fn log_network(&self) {
        let mut info_str = "Path trace \n\n".to_string();
        for (node_id, node) in &self.network {
            info_str.push_str(format!("\n\t {node_id} => ").as_str());
            let iter = node.neighbours.borrow();
            for v in iter.iter() {
                info_str.push_str(format!(" {v}").as_str());
            } 
        }
        info!(
            "[RouterOf: {}] == {info_str}", self.root
        ) ;
    }
}

/// Returns a path from the vector of parents
/// # Errors
/// - `Err(ParentsMalformed)` if the vector of parents is malformed
fn parents_to_path(parents: &HashMap<NodeId, Option<NodeId>>, destination: NodeId) -> Result<Path> {
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
