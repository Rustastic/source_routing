use crate::error::Result;
use flood_requester::{neighbour::NeighBour, FloodRequester};
use network::Network;
use wg_2024::{
    network::{NodeId, SourceRoutingHeader},
    packet::{FloodResponse, NodeType},
};

mod network;
mod flood_requester;

#[derive(Debug)]
struct Router {
    id: NodeId,
    node_type: NodeType,
    network: Network,
    requester: FloodRequester,
}

impl Router {
    pub fn new_with_neighbours(
        id: NodeId,
        node_type: NodeType,
        neighbours: Vec<NeighBour>,
    ) -> Self {
        let requester = FloodRequester::new(neighbours, id);
        let network = Network::new(id, node_type);
        Self {
            id,
            node_type,
            network,
            requester,
        }
    }
    pub fn handle_flood_response(&mut self, resp: &FloodResponse) {
        self.network.update_from_path_trace(&resp.path_trace);
    }
    pub fn get_source_routing_header(&self, destination: NodeId) -> Result<SourceRoutingHeader> {
        let path = self.network.get_routes(destination)?;
        let header = SourceRoutingHeader::initialize(path);
        Ok(header.without_loops())
    }
}
