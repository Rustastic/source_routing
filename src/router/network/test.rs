use super::*;

/*
    1 => 2 => 3 => 4
    2 => 4
    3 => 1






*/

#[test]
fn test_bfs() {
    let mut network = Network::new(1, NodeType::Client) ;
    network.add_empty_node(2 , NodeType::Drone);
    network.add_empty_node(3 , NodeType::Drone);
    network.add_empty_node(4 , NodeType::Server);
    // 1 => 2 => 3 => 4
    // 2 => 4
    // 3 => 1
    network.add_link(1, 2);
    network.add_link(2, 3);
    network.add_link(3, 4);
    network.add_link(2, 4);
    network.add_link(3, 1);

    println!("{network:?}") ;

    let path = network.get_routes(4).unwrap() ;
    assert_eq!(path, vec![1, 2, 4]) ;
}