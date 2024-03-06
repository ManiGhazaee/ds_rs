#![cfg(test)]

use ds_rs::graph::{Edge, Graph, InsertEdgeErr, Node};

#[test]
fn test_basic() {
    let mut g: Graph<usize, char, usize> = Graph::new();
    assert_eq!(g.nodes_len(), 0);
    assert_eq!(g.edges_len(), 0);

    g.insert(Node::new(1, '3', []));

    assert_eq!(g.get(1), Some(&Node::new(1, '3', [])));
    assert!(g.get(1).unwrap().neighbors().is_empty());
    assert_eq!(g.get(1).unwrap().val(), &'3');
    assert_eq!(g.get(1).unwrap().key(), &1);
    assert_eq!(g.nodes_len(), 1);
    assert_eq!(g.edges_len(), 0);

    g.get_mut(1).unwrap().insert_neighbor(2, 100);

    assert_eq!(g.nodes_len(), 1);
    assert_eq!(g.edges_len(), 1);
    assert!(!g.get(1).unwrap().neighbors().is_empty());
    assert_eq!(
        g.get(1)
            .unwrap()
            .neighbors()
            .iter()
            .collect::<Vec<(&usize, &usize)>>(),
        vec![(&2, &100)]
    );
    assert_eq!(g.get(1).unwrap().neighbors_as_vec(), vec![(&2, &100)]);
    assert_eq!(g.contains(2), false);
    assert_eq!(g.nodes_len(), 1);
    assert_eq!(g.edges_len(), 1);

    match g.insert_edge(2, 1, 200) {
        Err(InsertEdgeErr::FromNone) => (),
        _ => panic!(),
    }

    g.insert(Node::new(2, '8', [(1, 200)]));

    assert_eq!(g.get(2).unwrap().neighbors_as_vec(), vec![(&1, &200)]);
    let edges = g.edges();
    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&Edge::new(&1, &2, &100)));
    assert!(edges.contains(&Edge::new(&2, &1, &200)));
    assert_eq!(g.nodes_len(), 2);
    assert_eq!(g.edges_len(), 2);
}

#[test]
fn test_iter() {
    let mut g = Graph::new();

    g.insert(Node::new(0, 0, [(0, 0)]));
    g.insert(Node::new(1, 1, [(0, 0)]));
    g.insert(Node::new(2, 2, [(0, 0)]));
    g.insert(Node::new(3, 3, [(0, 0)]));
    g.insert(Node::new(4, 4, [(0, 0)]));
    g.insert(Node::new(5, 5, [(0, 0)]));

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
}
