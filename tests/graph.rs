#![cfg(test)]

use ds_rs::graph::hash_map::{Edge, EdgeErr, Graph, Node};

#[test]
fn test_basic() {
    let mut g: Graph<usize, char, usize> = Graph::new();
    assert_eq!(g.nodes_len(), 0);
    assert_eq!(g.edges_len(), 0);

    g.insert(Node::new(1, '3'));

    assert_eq!(g.get(&1), Some(&Node::new(1, '3')));
    assert!(g.get(&1).unwrap().neighbors().is_empty());
    assert_eq!(g.get(&1).unwrap().val(), &'3');
    assert_eq!(g.get(&1).unwrap().key(), &1);
    assert_eq!(g.nodes_len(), 1);
    assert_eq!(g.edges_len(), 0);

    match g.insert_edge(2, 1, 200) {
        Err(EdgeErr::FromNone) => (),
        _ => panic!(),
    }
    match g.insert_edge(1, 2, 200) {
        Err(EdgeErr::ToNone) => (),
        _ => panic!(),
    }

    g.insert(Node::new(2, '8'));
    g.insert_edge(2, 1, 200).unwrap();
    g.insert_edge(1, 2, 400).unwrap();

    assert_eq!(g.get(&2).unwrap().neighbors_as_vec(), vec![(&1, &200)]);
    assert_eq!(g.get(&1).unwrap().neighbors_as_vec(), vec![(&2, &400)]);
    let edges = g.edges();
    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&Edge::new(&2, &1, &200)));
    assert!(edges.contains(&Edge::new(&1, &2, &400)));
    assert_eq!(g.nodes_len(), 2);

    assert_eq!(g.get_weight(2, 1).unwrap(), &200);
    assert_eq!(g.get_weight(1, 2).unwrap(), &400);
}

#[test]
fn test_iter() {
    let mut g: Graph<i32, i32, i32> = Graph::new();

    g.insert(Node::new(0, 0));
    g.insert(Node::new(1, 1));
    g.insert(Node::new(2, 2));
    g.insert(Node::new(3, 3));
    g.insert(Node::new(4, 4));
    g.insert(Node::new(5, 5));

    let mut res = Vec::new();
    let mut res_nodes = Vec::new();
    for (k, t) in g.iter() {
        res.push(*k);
        res_nodes.push(t);
    }
    assert!(res.contains(&0));
    assert!(res.contains(&1));
    assert!(res.contains(&2));
    assert!(res.contains(&3));
    assert!(res.contains(&4));
    assert!(res.contains(&5));
    assert!(!res.contains(&6));

    assert_eq!(res_nodes.len(), 6);

    for (k, v) in g {
        println!("{}", k);
        println!("{}", v);
    }
}

#[test]
fn test_get_weight() {
    let mut g: Graph<i32, i32, i32> = Graph::new();

    g.insert(Node::new(0, 2));
    g.insert(Node::new(1, 4));

    g.insert_edge(0, 1, -2).unwrap();
    g.insert_edge(1, 0, -4).unwrap();

    assert_eq!(g.get_weight(0, 1).unwrap(), &-2);
    assert_eq!(g.get_weight(1, 0).unwrap(), &-4);
    match g.get_weight(1, 1) {
        Err(EdgeErr::ToNone) => (),
        _ => panic!(),
    }
}

#[test]
fn test_insert_node() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node = Node::new(1, 10);
    graph.insert(node.clone());
    assert_eq!(graph.nodes_len(), 1);
    assert_eq!(graph.get(&1), Some(&node));
}

#[test]
fn test_remove_node() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node = Node::new(1, 10);
    graph.insert(node.clone());
    assert_eq!(graph.nodes_len(), 1);
    assert_eq!(graph.remove(1), Some(node.clone()));
    assert_eq!(graph.nodes_len(), 0);
}

#[test]
fn test_contains_node() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node = Node::new(1, 10);
    graph.insert(node.clone());
    assert!(graph.contains(&1));
    assert!(!graph.contains(&2));
}

#[test]
fn test_insert_edge() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node1 = Node::new(1, 10);
    let node2 = Node::new(2, 20);
    graph.insert(node1.clone());
    graph.insert(node2.clone());
    graph.insert_edge(1, 2, 5).unwrap();
    assert_eq!(graph.edges_len(), 1);
    assert_eq!(graph.get_weight(1, 2), Ok(&5));
}

#[test]
fn test_remove_edge() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node1 = Node::new(1, 10);
    let node2 = Node::new(2, 20);
    graph.insert(node1.clone());
    graph.insert(node2.clone());
    graph.insert_edge(1, 2, 5).unwrap();
    assert_eq!(graph.edges_len(), 1);
    assert_eq!(graph.remove_edge(1, 2), Ok(Some(5)));
    assert_eq!(graph.edges_len(), 0);
}

#[test]
fn test_get_weight_mut() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node1 = Node::new(1, 10);
    let node2 = Node::new(2, 20);
    graph.insert(node1.clone());
    graph.insert(node2.clone());
    graph.insert_edge(1, 2, 5).unwrap();
    assert_eq!(graph.get_weight_mut(1, 2), Ok(&mut 5));
}

#[test]
fn test_error_conditions() {
    let mut graph = Graph::<i32, i32, i32>::new();
    let node1 = Node::new(1, 10);
    graph.insert(node1.clone());

    assert_eq!(graph.insert_edge(2, 1, 5), Err(EdgeErr::FromNone));
    assert_eq!(graph.insert_edge(1, 2, 5), Err(EdgeErr::ToNone));
    assert_eq!(graph.remove_edge(1, 2), Err(EdgeErr::ToNone));
    assert_eq!(graph.get_weight(1, 2), Err(EdgeErr::ToNone));
    assert_eq!(graph.get_weight_mut(1, 2), Err(EdgeErr::ToNone));

    assert_eq!(graph.remove(2), None);
    assert_eq!(graph.get(&2), None);
    assert!(!graph.contains(&2));
}

#[test]
fn test_dfs_iter() {
    let mut graph = Graph::<i32, &str, i32>::new();
    let node1 = Node::new(1, "1");
    let node2 = Node::new(2, "2");
    let node3 = Node::new(3, "3");
    let node4 = Node::new(4, "4");

    graph.insert(node1);
    graph.insert(node2);
    graph.insert(node3);
    graph.insert(node4);

    graph.insert_edge(1, 2, 10).unwrap();
    graph.insert_edge(1, 3, 20).unwrap();
    graph.insert_edge(2, 4, 30).unwrap();

    let dfs_iter = graph.dfs_iter(&1);

    let visited_nodes: Vec<Node<i32, &str, i32>> = dfs_iter.map(|i| i.to_owned()).collect();

    assert_eq!(visited_nodes.len(), 4);
}

#[test]
fn test_bfs_iter() {
    let mut graph = Graph::<i32, &str, i32>::new();
    let node1 = Node::new(1, "1");
    let node2 = Node::new(2, "2");
    let node3 = Node::new(3, "3");
    let node4 = Node::new(4, "4");

    graph.insert(node1);
    graph.insert(node2);
    graph.insert(node3);
    graph.insert(node4);

    graph.insert_edge(1, 2, 10).unwrap();
    graph.insert_edge(1, 3, 20).unwrap();
    graph.insert_edge(2, 4, 30).unwrap();

    let bfs_iter = graph.bfs_iter(&1);

    let visited_nodes: Vec<Node<i32, &str, i32>> = bfs_iter.map(|i| i.to_owned()).collect();

    assert_eq!(visited_nodes.len(), 4);
}

#[test]
fn test_find_eulerian_path() {
    let mut g: Graph<char, usize, usize> = Graph::new();
    g.insert(Node::new('A', 0));
    g.insert(Node::new('B', 0));
    g.insert(Node::new('C', 0));
    g.insert(Node::new('D', 0));
    g.insert_edge('A', 'B', 1).unwrap();
    g.insert_edge('B', 'C', 1).unwrap();
    g.insert_edge('C', 'D', 1).unwrap();
    g.insert_edge('D', 'B', 1).unwrap();

    let eulerian_path = g.find_eulerian_path().unwrap();
    assert_eq!(eulerian_path, vec![&'A', &'B', &'C', &'D', &'B']);
}
