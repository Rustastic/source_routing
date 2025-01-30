use std::collections::HashMap;

use wg_2024::{network::NodeId, packet::NodeType};

pub type Result<T> = std::result::Result<T, RouterError>;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum RouterError {
    IdNotFound {
        id: NodeId,
    },
    IdAlreadyPresent {
        id: NodeId,
        node_type: NodeType,
    },
    RemoveSelfErr,
    RouteNotFound {
        destination: NodeId,
    },
    ParentsMalformed {
        parents: HashMap<NodeId, Option<NodeId>>,
        destination: NodeId,
    },
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
