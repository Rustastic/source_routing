use std::
    collections::HashSet
;

use crate::error::Result;
use flood_requester::FloodRequestFactory;
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
    network: Network,
    requester: FloodRequestFactory,
}

impl Router {
    //constructors
    #[must_use]
    pub fn new(
        id: NodeId,
        node_type: NodeType, /* neighbours: &HashMap<NodeId, Sender<Packet>> */
    ) -> Self {
        let requester = FloodRequestFactory::new(/* neighbours.clone() ,*/ id, node_type);
        let network = Network::new(id, node_type);
        Self {
            id,
            node_type,
            network,
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
        self.network.update_from_path_trace(&resp.path_trace);
    }
    /// # Errors
    /// - `Err(RouteNotFound)` if the destionation is unreachable
    pub fn get_source_routing_header(&self, destination: NodeId) -> Result<SourceRoutingHeader> {
        self.network.log_network();
        let path = self.network.get_routes(destination)?;
        let header = SourceRoutingHeader::with_first_hop(path);
        Ok(header.without_loops())
    }
    pub fn log_network(&self) {
        self.network.log_network();
    }

    pub fn get_multiple_source_routing_headers(
        &self,
        destination: NodeId,
    ) -> Vec<SourceRoutingHeader> {
        let paths = self.network.multiple_paths(destination);
        let mut source_routing_headers = Vec::new();
        for path in paths {
            source_routing_headers.push(SourceRoutingHeader::initialize(path));
        }
        source_routing_headers
    }

    pub fn get_flood_request(&self) -> Packet {
        self.requester.get_flood_request()
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
        self.network.remove_node(id).map(|_| ())
        // .and(self.requester.remove_neighbour(id))
    }
    /// # Errors
    /// - `Err(IdAlreadyPresent)` with `node_type` set to `NodeType::Drone`
    ///   (assuming a client does not have neighbours not Drone)
    pub fn add_neighbour(&mut self, id: NodeId) -> Result<()> {
        self.network.add_neighbour(id)
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
        let server_list = self.network.get_server_list();
        log::info!("{:?}", server_list);
        server_list
    }
    pub fn clear_routing_table(&mut self) {
        self.network = Network::new(self.id, self.node_type);
    }
}
