use crossbeam_channel::Sender;
use wg_2024::{network::NodeId, packet::Packet};

#[derive(Debug)]
pub struct NeighBour {
    id: NodeId,
    sender: Sender<Packet>,
}

impl NeighBour {
    pub fn new(id: NodeId, sender: Sender<Packet>) -> Self {
        Self { id, sender }
    }
    pub fn send_request(&self, request: Packet) {
        let _ = self
            .sender
            .send(request)
            .inspect_err(|_e| todo!("send to sim controller"));
    }
    pub fn id(&self) -> NodeId {
        self.id
    }
}
