use wg_2024::{network::NodeId, packet::NodeType};

pub type Result<T> = std::result::Result<T, RouterError>;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum RouterError {
    IdNotFound { id: NodeId },
    IdAlreadyPresent { id: NodeId, node_type: NodeType },
    RemoveSelfErr,
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
