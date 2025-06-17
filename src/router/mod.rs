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
pub struct Router {
    id: NodeId,
    node_type: NodeType,
    primary_network: Network,
    secondary_network: Network,
    requester: FloodRequestFactory,
}

impl Router {
    //constructors
    #[must_use]
    pub fn new(id: NodeId, node_type: NodeType) -> Self {
        let requester = FloodRequestFactory::new(id, node_type);
        let network = Network::new(id, node_type);
        Self {
            id,
            node_type,
            primary_network: network.clone(),
            secondary_network: network,
            requester,
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
        self.primary_network
            .update_from_path_trace(&resp.path_trace);
    }
    /// # Errors
    /// - `Err(RouteNotFound)` if the destionation is unreachable
    pub fn get_source_routing_header(&self, destination: NodeId) -> Result<SourceRoutingHeader> {
        let path = self.primary_network.get_routes(destination)?;
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
    pub fn get_flood_requests(&self, count: usize) -> Vec<Packet> {
        self.requester.get_flood_request(count)
    }
    /*
    pub fn flood_neighbours(&self) {
    self.requester.flood_neighbours();
    }
    /// # Errors
    /// - `Err(IdNotFound)` if the `id` is not in the neighbour
    pub fn flood_with_id(&self, id: NodeId) -> Result<()> {
        self.requester.flood_with_id(id)
    }
    */
    /// # Errors
    /// - `Err(RemoveSelfErr)` if the id is the root
    /// - `Err(IdNotFound)`
    pub fn drone_crashed(&mut self, id: NodeId) -> Result<()> {
        self.primary_network.remove_node(id).map(|_| ())
        // .and(self.requester.remove_neighbour(id))
    }
    pub fn dropped_fragment(&mut self, id1: NodeId) {
        let _ = self.primary_network.increment_weight(id1);
    }
    /// # Errors
    /// - `Err(IdAlreadyPresent)` with `node_type` set to `NodeType::Drone`
    ///   (assuming a client does not have neighbours not Drone)
    pub fn add_neighbour(&mut self, id: NodeId) -> Result<()> {
        self.primary_network.add_neighbour(id)
        // self.requester
        //     .add_neighbour(id, sender)
        //     .and(self.network.add_neighbour(id))
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
        self.primary_network.get_server_list()
    }
    pub fn clear_routing_table(&mut self) {
        self.primary_network = Network::new(self.id, self.node_type);
    }
}
