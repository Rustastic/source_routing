	/*   <source_routing - lib for Rustastic client and server>     Copyright (C) <2025>  <Giulio Bosio>      This program is free software: you can redistribute it and/or modify     it under the terms of the GNU General Public License as published by     the Free Software Foundation, either version 3 of the License, or     (at your option) any later version.      This program is distributed in the hope that it will be useful,     but WITHOUT ANY WARRANTY; without even the implied warranty of     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the     GNU General Public License for more details.      You should have received a copy of the GNU General Public License     along with this program.  If not, see <https://www.gnu.org/licenses/>.  	*/

use wg_2024::packet::NodeType;

use wg_2024::network::NodeId;

use std::cell::RefCell;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub(crate) neighbours: RefCell<Vec<NodeId>>,
    pub(crate) node_type: NodeType,
}

impl NetworkNode {
    //constructor
    pub(crate) fn new(node_type: NodeType) -> Self {
        Self {
            neighbours: RefCell::new(Vec::new()),
            node_type,
        }
    }
}

impl NetworkNode {
    //methods
    /// # Note
    /// Does not check if the id is valid, so you have to ensure that the id is already in the network
    pub(crate) fn add_neighbour(&self, id: NodeId) {
        if !self.neighbours.borrow().contains(&id) {
            self.neighbours.borrow_mut().push(id);
        }
    }
    /// # Note
    /// Does not preserve order in the vector
    pub(crate) fn remove_neighbour(&self, id: NodeId) {
        let index = self.neighbours.borrow().iter().position(|&i| i == id);
        if let Some(index) = index {
            if let Ok(mut a) = self.neighbours.try_borrow_mut() {
                a.swap_remove(index);
            }
        }
    }
    /* /// Add some ids to the neightbours calling `std::vec::reserve()` before
    /// # Note
    /// Does not check if the ids are valid, so you have to ensure that the ids are already in the network
    fn bulk_add_neighbours(&self, ids: Vec<NodeId>) {
        self.neighbours.borrow_mut().reserve(ids.len());
        for id in ids {
            self.add_neighbour(id);
        }
    } */
}

