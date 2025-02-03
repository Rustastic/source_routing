use std::collections::HashMap;
use wg_2024::{
    network::NodeId,
    packet::{NodeType, Packet},
};

pub type Result<T> = std::result::Result<T, Box<RouterError>>;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum RouterError {
    IdNotFound(NodeId),
    RemoveSelfErr,
    IdAlreadyPresent {
        id: NodeId,
        node_type: NodeType,
    },
    RouteNotFound {
        destination: NodeId,
    },
    ParentsMalformed {
        parents: HashMap<NodeId, Option<NodeId>>,
        destination: NodeId,
    },
    SendError {
        destination: NodeId,
        error: crossbeam_channel::SendError<Packet>,
    },
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
