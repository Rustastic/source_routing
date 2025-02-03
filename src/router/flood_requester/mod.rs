use crate::error::{
    Result,
    RouterError::{IdAlreadyPresent, IdNotFound},
};
use neighbour::NeighBour;
use std::cell::RefCell;
use wg_2024::{
    network::{NodeId, SourceRoutingHeader},
    packet::{FloodRequest, Packet, PacketType},
};

pub mod neighbour;

#[derive(Debug)]
pub struct FloodRequester {
    neighbours: Vec<NeighBour>,
    flood_ids: RefCell<Vec<u64>>,
    id: NodeId,
}

impl FloodRequester {
    //constructor
    pub fn new(neighbour_channel: Vec<NeighBour>, id: NodeId) -> Self {
        Self {
            neighbours: neighbour_channel,
            flood_ids: RefCell::new(Vec::new()),
            id,
        }
    }
}

impl FloodRequester {
    //methods
    pub fn flood_neighbours(&self) {
        for neighbour in &self.neighbours {
            let flood_request = self.create_request();
            let packet = flood_request_to_packet(flood_request);
            neighbour.send_request(packet);
        }
    }
    /// send a `flood request` only to `id`
    /// # Errors
    /// - `IdNotFound` if the `id` is not in the neighbours
    pub fn flood_with_id(&self, id: NodeId) -> Result<()> {
        let target = self
            .neighbours
            .iter()
            .find(|&n| n.id() == id)
            .ok_or(IdNotFound(id))?;
        let flood_request = self.create_request();
        let packet = flood_request_to_packet(flood_request);
        target.send_request(packet);
        Ok(())
    }
    /// # Note
    ///  Does not preserve the order of the vec
    /// # Errors
    /// - `Err(IdNotFound)` if the id is not a neighbour
    pub fn remove_neighbour(&mut self, id: NodeId) -> Result<()> {
        if let Some(index) = self.neighbours.iter().position(|i| id == i.id()) {
            self.neighbours.swap_remove(index);
            Ok(())
        } else {
            Err(IdNotFound(id))
        }
    }
    /// # Errors 
    /// - `Err(IdAlreadyPresent)` with `node_type` set to `NodeType::Drone`
    ///   (assuming a client does not have neighbours not Drone)
    pub fn add_neighbour(&mut self, neighbour: NeighBour) -> Result<()> {
        if self.contains_id(neighbour.id()) {
            return Err(IdAlreadyPresent {
                id: neighbour.id(),
                node_type: wg_2024::packet::NodeType::Drone,
            });
        }
        self.neighbours.push(neighbour);
        Ok(())
    }
    fn contains_id(&self, id: NodeId) -> bool {
        self.neighbours.iter().any(|n| n.id() == id)
    }
    fn create_request(&self) -> FloodRequest {
        let flood_id = self
            .flood_ids
            .borrow()
            .last()
            .map_or_else(|| 0, |id| id + 1);

        self.flood_ids.borrow_mut().push(flood_id);
        FloodRequest::new(flood_id, self.id)
    }
}

fn flood_request_to_packet(req: FloodRequest) -> Packet {
    Packet {
        routing_header: SourceRoutingHeader::empty_route(),
        session_id: 0, //to be ignored
        pack_type: PacketType::FloodRequest(req),
    }
}
