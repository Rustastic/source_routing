use std::collections::HashSet;

use crate::error::Result;
use network::Network;
use wg_2024::{
    network::NodeId,
    packet::{FloodResponse, NodeType},
};

mod network;

#[derive(Debug)]
enum NetworkStatus {
    RequestSended,
    ResponseReceived(usize),
    Swapped,
}

#[derive(Debug)]
pub struct NetworkHolder {
    id: NodeId,
    node_type: NodeType,
    primary_network: Network,
    secondary_network: Network,
    status: NetworkStatus,
}

impl NetworkHolder {
    //constructor
    #[must_use]
    pub fn new(root: NodeId, root_type: NodeType) -> Self {
        Self {
            id: root,
            node_type: root_type,
            primary_network: Network::new(root, root_type),
            secondary_network: Network::new(root, root_type),
            status: NetworkStatus::ResponseReceived(0),
        }
    }
}

impl NetworkHolder {
    //methods
    pub fn received_flood_response(&mut self, response: &FloodResponse) {
        match self.status {
            NetworkStatus::RequestSended => {
                self.status = NetworkStatus::ResponseReceived(1);
                self.secondary_network = Network::new(self.id, self.node_type);
            }
            NetworkStatus::ResponseReceived(count)
                if count > self.primary_network.get_node_number() * 60 / 100 =>
            {
                self.swap_network();
            }
            NetworkStatus::ResponseReceived(count) => {
                self.status = NetworkStatus::ResponseReceived(count + 1);
            }
            NetworkStatus::Swapped => {}
        }
        self.primary_network
            .update_from_path_trace(&response.path_trace);
        self.secondary_network
            .update_from_path_trace(&response.path_trace);
    }
    pub fn asked_flood_request(&mut self) {
        if !matches!(self.status, NetworkStatus::Swapped) {
            self.swap_network();
        }
        self.status = NetworkStatus::RequestSended;
    }
    pub fn drone_crashed(&mut self, id: NodeId) {
        let _ = self.primary_network.remove_node(id);
        let _ = self.secondary_network.remove_node(id);
    }
    pub fn dropped_fragment(&mut self, id1: NodeId) {
        let _ = self.primary_network.increment_weight(id1);
        let _ = self.secondary_network.increment_weight(id1);
    }
    pub fn add_neighbour(&mut self, id: NodeId) {
        let _ = self.primary_network.add_neighbour(id);
        let _ = self.secondary_network.add_neighbour(id);
    }
    pub fn remove_neighbour(&mut self, id: NodeId) {
        let _ = self.primary_network.remove_neighbour_link(id);
        let _ = self.secondary_network.remove_neighbour_link(id);
    }
    /// Returns the list of server in the network, used to determine which server is Chat
    /// and which is Media/Text  
    pub fn get_server_list(&self) -> HashSet<NodeId> {
        self.primary_network
            .get_server_list()
            .into_iter()
            .chain(self.secondary_network.get_server_list())
            .collect()
    }
    fn swap_network(&mut self) {
        std::mem::swap(&mut self.primary_network, &mut self.secondary_network);
        self.status = NetworkStatus::Swapped;
    }
}

impl NetworkHolder {
    // getter setter
    pub fn get_path(&self, destination: NodeId) -> Result<Vec<NodeId>> {
        let path = self
            .primary_network
            .get_routes(destination)
            .or_else(|_| self.secondary_network.get_routes(destination))?;
        Ok(path)
    }
    pub fn get_multiple_paths(&self, destination: NodeId) -> Vec<Vec<NodeId>> {
        self.primary_network.multiple_paths(destination)
    }
}
