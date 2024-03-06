#![cfg(test)]

use ds_rs::graph::{Edge, Graph, InsertEdgeErr, Node};

#[test]
fn test_basic() {
    let mut g: Graph<usize, char, usize> = Graph::new();
    g.insert(Node::new(1, Some('3'), []));

    assert_eq!(g.get(1), Some(&Node::new(1, Some('3'), [])));
    assert!(g.get(1).unwrap().neighbors().is_empty());
    assert_eq!(g.get(1).unwrap().val(), Some(&'3'));
    assert_eq!(g.get(1).unwrap().key(), &1);

    g.get_mut(1).unwrap().insert_neighbor(2, 100);

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

    match g.insert_edge(2, 1, 200) {
        Err(InsertEdgeErr::FromNone) => (),
        _ => panic!()
    }

    g.insert(Node::new(2, Some('8'), [(1, 200)]));

    assert_eq!(g.get(2).unwrap().neighbors_as_vec(), vec![(&1, &200)]);
    let edges = g.edges();
    assert_eq!(edges.len(), 2);
    assert!(edges.contains(&Edge::new(&1, &2, &100)));
    assert!(edges.contains(&Edge::new(&2, &1, &200)));

}
