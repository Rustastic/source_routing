use wg_2024::network::NodeId;

use crate::flood_requester::{FloodRequester, neighbour::NeighBour};

#[derive(Debug)]
struct Router<'a> {
    requester: FloodRequester<'a>,
    id: NodeId,
}

impl<'a> Router<'a> {
    fn new(id: NodeId, neighbour: Vec<NeighBour<'a>>) -> Self {
        let requester = FloodRequester::new(neighbour, id);
        Self { requester, id }
    }
}
