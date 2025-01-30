use network::Network;
use wg_2024::{
    network::NodeId,
    packet::{FloodResponse, NodeType},
};

use crate::flood_requester::{neighbour::NeighBour, FloodRequester};

mod network;

#[derive(Debug)]
struct Router<'a> {
    id: NodeId,
    node_type: NodeType,
    network: Network,
    requester: FloodRequester<'a>,
}

impl<'a> Router<'a> {
    pub fn new_with_neighbours(
        id: NodeId,
        node_type: NodeType,
        neighbours: Vec<NeighBour<'a>>,
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
    pub fn receveid_flood_response(&mut self, resp: &FloodResponse) {
        self.network.update_from_path_trace(&resp.path_trace);
    }
}
