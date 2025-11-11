# source_routing

**source_routing** is a Rust library that implements a *source routing* mechanism for message delivery in a distributed network of nodes.  
This project was developed as part of the **Advanced Programming** course (A.Y. 2024–2025) at the **University of Trento**.

---

## 🎯 Project Overview

The goal of this project was to design and implement the routing logic of a distributed network simulator, focusing on:
- Dynamic topology discovery through *flooding*;
- Path computation based on **Dijkstra’s less expensive path algorithm**;
- Integration with the `wg_2024`, which provides data structures and message formats at the network layer.

Dijkstra is the core algorithm used for route computation in this version; also a **bfs** is present

---

## 🧠 Design Highlights

### 1. Router Architecture
The `Router` acts as the main logic unit responsible for:
- Maintaining an internal graph representation of the network;
- Updating the topology and the network status when receiving *FloodRequests* and *FloodResponses*;
- Constructing routing headers and determining forwarding paths for outgoing packets.

### 2. Flood-Based Topology Discovery
Each router learns the network structure using a distributed *flooding mechanism*:
- Nodes issue `FloodRequest` packets containing their ID and a sequence number;
- Neighboring nodes forward the request, ensuring full network coverage without loops;
- Upon receiving `FloodResponse` messages, routers construct their local network view.

### 3. Network Representation
The network is internally modeled as a **weighted graph**:
- Nodes are represented by `NodeId: u8` (from `wg_2024`);
- Edges represent direct connections between neighboring nodes;
- Each edge has a weight, dynamically adjusted to reflect reliability; the weight is the number of the dropped packet on the link.

### 4. Route Computation (Dijkstra)
Routing logic is based on **Dijkstra’s algorithm** for computing the shortest path between two nodes in terms of link weight, to avoid link that drop packet.

### 5. Dual-Network Management
The `NetworkHolder` structure maintains two copies of the network:
- A **primary** network for real-time routing decisions;
- A **secondary** network updated in parallel and swapped when appropriate. Used also when a node is unreachable.

---

## 🧩 Repository Structure

```
src/
 ├── lib.rs                          # Library entry point and public exports
 ├── error.rs                        # Custom error types
 └── router/        
      ├── mod.rs                     # Main Router implementation
      ├── flood_requester/           # Logic for generating and sending flood packets
      └── network_holder/            # Dual-network management and topology updates
           ├── mod.rs        
           └── network/        
                ├── mod.rs           # Graph representation and Dijkstra implementation
                ├── network_node.rs 
                └── test.rs          # Unit tests
```

---

## ⚙️ Technical Stack

- **Language:** Rust (edition 2021)  
- **External library:** [`wg_2024`](https://github.com/WGL-2024/WGL_repo_2024)  
- **Dependencies:**  
  - `crossbeam-channel`  
  - `log`, `env_logger`  
  - `rand`  
  - `toml`  
  - `colored`  
  - `priority-queue`  

---

## 🧪 Testing and Validation

Unit tests verify:
- Correct network graph construction and updates;  
- Dijkstra’s shortest-path results on known topologies;  
- Proper handling of flooding requests and responses;  

Run tests with:

```
> cargo test
```

---

## 📚 Acknowledgements

Developed for the *Advanced Programming* course,  
**University of Trento, A.Y. 2024–2025**

---

## 🪪 License

This project is released under the **GNU General Public License v3.0 (GPLv3)**.  
See [https://www.gnu.org/licenses/gpl-3.0.html](https://www.gnu.org/licenses/gpl-3.0.html).

© 2025 Giulio Bosio
