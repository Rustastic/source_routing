use crate::error::{
    Result,
    RouterError::{IdAlreadyPresent, IdNotFound, SendError},
};
use crossbeam_channel::Sender;
use std::{cell::RefCell, collections::HashMap};
use wg_2024::{
    network::{NodeId, SourceRoutingHeader},
    packet::{FloodRequest, Packet, PacketType},
};

// use neighbour::NeighBour;
// pub mod neighbour;

#[derive(Debug)]
pub struct FloodRequestFactory {
    // neighbours: Vec<NeighBour>,
    // flood_send: HashMap<NodeId, Sender<Packet>>,
    flood_ids: RefCell<Vec<u64>>,
    id: NodeId,
}

impl FloodRequestFactory {
    //constructor
    pub fn new(/* neighbour_channel: HashMap<NodeId, Sender<Packet>>, */ id: NodeId) -> Self {
        Self {
            // flood_send: neighbour_channel,
            flood_ids: RefCell::new(Vec::new()),
            id,
        }
    }
}

impl FloodRequestFactory {
    //methods
    // pub fn flood_neighbours(&self) -> Vec<Result<()>> {
    //     self.flood_send
    //         .iter()
    //         .map(|(id, sender)| {
    //             let flood_request = self.create_request();
    //             let packet = flood_request_to_packet(flood_request);
    //             sender.send(packet).map_err(|e| {
    //                 Box::new(SendError {
    //                     destination: *id,
    //                     error: e,
    //                 })
    //             })
    //         })
    //         .collect()
    // }
    pub fn get_flood_request(&self) -> Packet {
        self.flood_request_to_packet(self.create_request())
    }
    /// send a `flood request` only to `id`
    /// # Errors
    /// - `IdNotFound` if the `id` is not in the neighbours
    // pub fn flood_with_id(&self, id: NodeId) -> Result<()> {
    //     let flood_request = self.create_request();
    //     let packet = flood_request_to_packet(flood_request);
    //     self.flood_send
    //         .get(&id)
    //         .ok_or(IdNotFound(id))?
    //         .send(packet)
    //         .map_err(|e| {
    //             Box::new(SendError {
    //                 destination: id,
    //                 error: e,
    //             })
    //         })
    // }
    /// # Errors
    /// - `Err(IdNotFound)` if the id is not a neighbour
    // pub fn remove_neighbour(&mut self, id: NodeId) -> Result<()> {
    //     self.flood_send.remove(&id).ok_or(IdNotFound(id))?;
    //     Ok(())
    // }
    /// # Errors
    /// - `Err(IdAlreadyPresent)` with `node_type` set to `NodeType::Drone`
    ///   (assuming a client/server does not have neighbours not Drone)
    // pub fn add_neighbour(&mut self, id: NodeId, sender: Sender<Packet>) -> Result<()> {
    //     if self.contains_id(id) {
    //         return Err(Box::new(IdAlreadyPresent {
    //             id,
    //             node_type: wg_2024::packet::NodeType::Drone,
    //         }));
    //     }
    //     self.flood_send.insert(id, sender);
    //     Ok(())
    // }
    // fn contains_id(&self, id: NodeId) -> bool {
    //     self.flood_send.contains_key(&id)
    // }
    fn create_request(&self) -> FloodRequest {
        let flood_id = self
            .flood_ids
            .borrow()
            .last()
            .map_or_else(|| 0, |id| id + 1);

        self.flood_ids.borrow_mut().push(flood_id);
        FloodRequest::new(flood_id, self.id)
    }
    fn flood_request_to_packet(&self, req: FloodRequest) -> Packet {
        Packet {
            routing_header: SourceRoutingHeader::with_first_hop(vec![self.id]),
            session_id: 0, //to be ignored
            pack_type: PacketType::FloodRequest(req),
        }
    }
    
}
