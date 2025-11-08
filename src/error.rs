	/*   <source_routing - lib for Rustastic client and server>     Copyright (C) <2025>  <Giulio Bosio>      This program is free software: you can redistribute it and/or modify     it under the terms of the GNU General Public License as published by     the Free Software Foundation, either version 3 of the License, or     (at your option) any later version.      This program is distributed in the hope that it will be useful,     but WITHOUT ANY WARRANTY; without even the implied warranty of     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the     GNU General Public License for more details.      You should have received a copy of the GNU General Public License     along with this program.  If not, see <https://www.gnu.org/licenses/>.  	*/

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

