use std::cell::RefCell;

use neighbour::NeighBour;
use wg_2024::{
    network::{NodeId, SourceRoutingHeader},
    packet::{FloodRequest, Packet, PacketType},
};

pub mod neighbour;

#[derive(Debug)]
pub struct FloodRequester<'a> {
    neighbours: Vec<NeighBour<'a>>,
    flood_ids: RefCell<Vec<u64>>,
    id: NodeId,
}

impl<'a> FloodRequester<'a> {
    pub fn new(neighbour_channel: Vec<NeighBour<'a>>, id: NodeId) -> Self {
        Self {
            neighbours: neighbour_channel,
            flood_ids: RefCell::new(Vec::new()),
            id,
        }
    }
    pub fn flood_neighbours(&self) {
        for neighbour in &self.neighbours {
            let flood_request = self.create_request();
            let packet = flood_request_to_packet(flood_request);
            neighbour.send_request(packet);
        }
    }
    /// send a `flood request` only to `id`
    /// returns `None` if the `id` is not in the neighbours
    pub fn flood_with_id(&self, id: NodeId) -> Option<()> {
        let target = self.neighbours.iter().find(|&n| n.id() == id)?;
        let flood_request = self.create_request();
        let packet = flood_request_to_packet(flood_request);
        target.send_request(packet);
        Some(())
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