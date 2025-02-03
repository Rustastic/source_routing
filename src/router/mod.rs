use crate::error::Result;
use crossbeam_channel::Sender;
use flood_requester::{neighbour::NeighBour, FloodRequester};
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
    requester: FloodRequester,
}

impl Router {
    //constructors
    #[must_use] 
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
}

impl Router {
    //methods
    pub fn handle_flood_response(&mut self, resp: &FloodResponse) {
        self.network.update_from_path_trace(&resp.path_trace);
    }
    /// # Errors
    /// - `Err(RouteNotFound)` if the destionation is unreachable
    pub fn get_source_routing_header(&self, destination: NodeId) -> Result<SourceRoutingHeader> {
        let path = self.network.get_routes(destination)?;
        let header = SourceRoutingHeader::initialize(path);
        Ok(header.without_loops())
    }
    pub fn flood_neighbours(&self) {
        self.requester.flood_neighbours();
    }
    /// # Errors
    /// - `Err(IdNotFound)` if the `id` is not in the neighbour
    pub fn flood_with_id(&self, id: NodeId) -> Result<()> {
        self.requester.flood_with_id(id)
    }
    /// # Errors
    /// - `Err(RemoveSelfErr)` if the id is the root
    /// - `Err(IdNotFound)` 
    pub fn drone_crashed(&mut self, id: NodeId) -> Result<()> {
        // let _ = self.requester.remove_neighbour(id);
        self.network.remove_node(id).map(|_| ())
    }
    /// # Errors
    /// - `Err(IdAlreadyPresent)` with `node_type` set to `NodeType::Drone`
    ///   (assuming a client does not have neighbours not Drone)
    pub fn add_neighbour(&mut self, id: NodeId, sender: Sender<Packet>) -> Result<()> {
        let neighbour = NeighBour::new(id, sender) ;
        self.requester.add_neighbour(neighbour)
    }
    /// # Errors
    /// - `Err(IdNotFound)` if the id is not a neighbour
    pub fn remove_neighbour(&mut self, id: NodeId) -> Result<()> {
        self.requester.remove_neighbour(id) 
    }
}
