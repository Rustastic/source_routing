use std::cell::RefCell;

use crossbeam_channel::{Receiver, Sender};
use wg_2024::{
    network::NodeId,
    packet::{FloodRequest, FloodResponse},
};

#[derive(Debug)]
pub struct NeighBour {
    id: NodeId,
    sender: Sender<FloodRequest>,
    receiver: Receiver<FloodResponse>,
}

impl NeighBour {
    fn send_request(&self, request: FloodRequest) {
        let _ = self
            .sender
            .send(request)
            .inspect_err(|_e| todo!("send to sim controller"));
    }
}

#[derive(Debug)]
pub struct FloodRequester {
    neighbours: Vec<NeighBour>,
    flood_ids: RefCell<Vec<u64>>,
    id: NodeId,
}

impl FloodRequester {
    pub fn new(neighbour_channel: Vec<NeighBour>, id: NodeId) -> Self {
        Self {
            neighbours: neighbour_channel,
            flood_ids: RefCell::new(Vec::new()),
            id,
        }
    }
    pub fn flood_neighbours(&self) {
        for neighbour in &self.neighbours {
            let flood_request = self.create_request();
            neighbour.send_request(flood_request);
        }

        todo!()
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
