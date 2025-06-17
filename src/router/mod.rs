use std::collections::HashSet;

use crate::error::Result;
use flood_requester::FloodRequestFactory;
use log::info;
use network::Network;
use wg_2024::{
    network::{NodeId, SourceRoutingHeader},
    packet::{FloodResponse, NodeType, Packet},
};

mod flood_requester;
mod network;

#[derive(Debug)]
enum NetworkStatus {
    RequestSended,
    ResponseReceived(usize),
    Swapped,
}

#[derive(Debug)]
pub struct Router {
    id: NodeId,
    node_type: NodeType,
    primary_network: Network,
    secondary_network: Network,
    status: NetworkStatus,
    requester: FloodRequestFactory,
}

impl Router {
    //constructors
    #[must_use]
    pub fn new(id: NodeId, node_type: NodeType) -> Self {
        let network = Network::new(id, node_type);
        Self {
            id,
            node_type,
            primary_network: network.clone(),
            secondary_network: network,
            status: NetworkStatus::ResponseReceived(0),
            requester: FloodRequestFactory::new(id, node_type),
        }
    }
    /*
    #[must_use]
    pub fn new_with_hashmaps(id: NodeId, packet_send: HashMap<NodeId, Sender<Packet>>) -> Self {
        let mut neighbour = vec![];
        for (id, send) in packet_send {
            neighbour.push(NeighBour::new(id, send.clone()));
        }
        let requester = FloodRequester::new(neighbour, id);
        let network = Network::new(id, NodeType::Client);
        Self {
            id,
            node_type: NodeType::Client,
            network,
            requester,
        }
    }
    */
}

impl Router {
    //methods
    pub fn handle_flood_response(&mut self, resp: &FloodResponse) {
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
            .update_from_path_trace(&resp.path_trace);
        self.secondary_network
            .update_from_path_trace(&resp.path_trace);
    }
    /// # Errors
    /// - `Err(RouteNotFound)` if the destionation is unreachable
    pub fn get_source_routing_header(&self, destination: NodeId) -> Result<SourceRoutingHeader> {
        let path = self
            .primary_network
            .get_routes(destination)
            .or_else(|_| self.secondary_network.get_routes(destination))?;
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
        let paths = self.primary_network.multiple_paths(destination);
        let mut source_routing_headers = Vec::new();
        for path in paths {
            source_routing_headers.push(SourceRoutingHeader::initialize(path));
        }
        source_routing_headers
    }
    /// # Returns:
    /// A Vec<Packet> with the size specified in `count`
    pub fn get_flood_requests(&mut self, count: usize) -> Vec<Packet> {
        if !matches!(self.status, NetworkStatus::Swapped) {
            self.swap_network();
        }
        self.status = NetworkStatus::RequestSended;
        self.requester.get_flood_request(count)
    }
    pub fn drone_crashed(&mut self, id: NodeId) {
        let _ = self.primary_network.remove_node(id);
        let _ = self.secondary_network.remove_node(id);
    }
    pub fn dropped_fragment(&mut self, id1: NodeId) {
        let _ = self.primary_network.increment_weight(id1);
        let _ = self.secondary_network.increment_weight(id1);
    }
    /// # Errors
    /// - `Err(IdAlreadyPresent)` with `node_type` set to `NodeType::Drone`
    ///   (assuming a client does not have neighbours not Drone)
    pub fn add_neighbour(&mut self, id: NodeId) {
        let _ = self.primary_network.add_neighbour(id);
        let _ = self.secondary_network.add_neighbour(id);
    }
    /*
    /// # Errors
    /// - `Err(IdNotFound)` if the id is not a neighbour
    pub fn remove_neighbour(&mut self, id: NodeId) -> Result<()> {
        self.requester.remove_neighbour(id)
    }  */

    /// Returns the list of server in the network, used to determine which server is Chat
    /// and which is Media/Text  
    pub fn get_server_list(&self) -> HashSet<NodeId> {
        self.primary_network
            .get_server_list()
            .into_iter()
            .chain(self.secondary_network.get_server_list())
            .collect()
    }
    // pub fn clear_routing_table(&mut self) {
    //     self.primary_network = Network::new(self.id, self.node_type);
    // }
    fn swap_network(&mut self) {
        std::mem::swap(&mut self.primary_network, &mut self.secondary_network);
        self.status = NetworkStatus::Swapped;
    }
}
