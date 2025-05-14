use super::*;

#[test]
fn test_bfs() {
    let mut network = Network::new(1, NodeType::Client);
    let _ = network.add_empty_node(2, NodeType::Drone);
    let _ = network.add_empty_node(3, NodeType::Drone);
    let _ = network.add_empty_node(4, NodeType::Server);
    // 1 => 2 => 3 => 4
    // 2 => 4
    // 3 => 1
    let _ = network.add_link(1, 2);
    let _ = network.add_link(2, 3);
    let _ = network.add_link(3, 4);
    let _ = network.add_link(2, 4);
    let _ = network.add_link(3, 1);

    println!("{network:?}");

    let path = network.get_routes(4).unwrap();
    assert_eq!(path, vec![1, 2, 4]);
}

#[test]
fn test_remove_neighbour(){
    let node = NetworkNode::new(NodeType::Drone);
    node.add_neighbour(1);
    node.add_neighbour(2);
    node.add_neighbour(2);
    node.add_neighbour(3);

    node.remove_neighbour(12);
    node.add_neighbour(2);
}
