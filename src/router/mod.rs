use std::collections::HashSet;

use crate::error::Result;
use flood_requester::FloodRequestFactory;
use log::info;
use network_holder::NetworkHolder;
use wg_2024::{
    network::{NodeId, SourceRoutingHeader},
    packet::{FloodResponse, NodeType, Packet},
};

mod flood_requester;
mod network_holder;

#[derive(Debug)]
pub struct Router {
    id: NodeId,
    network_holder: NetworkHolder,
    requester: FloodRequestFactory,
}

impl Router {
    //constructors
    #[must_use]
    pub fn new(id: NodeId, node_type: NodeType) -> Self {
        Self {
            id,
            network_holder: NetworkHolder::new(id, node_type),
            requester: FloodRequestFactory::new(id, node_type),
        }
    }
}

impl Router {
    //methods
    pub fn handle_flood_response(&mut self, resp: &FloodResponse) {
        self.network_holder.received_flood_response(resp);
    }
    /// # Errors
    /// - `Err(RouteNotFound)` if the destionation is unreachable
    pub fn get_source_routing_header(&self, destination: NodeId) -> Result<SourceRoutingHeader> {
        let path: Vec<NodeId> = self.network_holder.get_path(destination)?;
        let header = SourceRoutingHeader::with_first_hop(path).without_loops();
        println!("[RouterOf: {}] header: {header}", self.id);
        info!("[RouterOf: {}] header: {header}", self.id);
        Ok(header)
    }
    pub fn log_network(&self) {
        // self.network.log_network();
    }

    pub fn get_multiple_source_routing_headers(
        &self,
        destination: NodeId,
    ) -> Vec<SourceRoutingHeader> {
        let paths = self.network_holder.get_multiple_paths(destination);
        let mut source_routing_headers = Vec::new();
        for path in paths {
            source_routing_headers.push(SourceRoutingHeader::initialize(path));
        }
        source_routing_headers
    }
    /// # Returns:
    /// A Vec<Packet> with the size specified in `count`
    pub fn get_flood_requests(&mut self, count: usize) -> Vec<Packet> {
        self.network_holder.asked_flood_request();
        self.requester.get_flood_request(count)
    }
    pub fn drone_crashed(&mut self, id: NodeId) {
        self.network_holder.drone_crashed(id);
    }
    pub fn dropped_fragment(&mut self, id1: NodeId) {
        self.network_holder.dropped_fragment(id1);
    }
    pub fn add_neighbour(&mut self, id: NodeId) {
        self.network_holder.add_neighbour(id);
    }

    pub fn remove_neighbour(&mut self, id: NodeId) {
        self.network_holder.remove_neighbour(id);
    }

    /// Returns the list of server in the network, used to determine which server is Chat
    /// and which is Media/Text  
    pub fn get_server_list(&self) -> HashSet<NodeId> {
        self.network_holder.get_server_list()
    }
    // pub fn clear_routing_table(&mut self) {
    //     self.primary_network = Network::new(self.id, self.node_type);
    // }
}
