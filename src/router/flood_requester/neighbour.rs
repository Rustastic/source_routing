use crossbeam_channel::Sender;
use wg_2024::{network::NodeId, packet::Packet};

#[derive(Debug)]
pub struct NeighBour {
    id: NodeId,
    sender: Sender<Packet>,
}

impl NeighBour {
    //constructor
    pub fn new(id: NodeId, sender: Sender<Packet>) -> Self {
        Self { id, sender }
    }
}
impl NeighBour {
    //methods
    pub fn send_request(&self, request: Packet) {
        let _ = self
            .sender
            .send(request)
            .inspect_err(|_e| todo!("send to sim controller"));
    }
}

impl NeighBour {
    //getter/setter
    pub fn id(&self) -> NodeId {
        self.id
    }
}
