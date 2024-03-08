use std::time::Instant;

use ds_rs::{
    graph::{self, Graph},
    matrix::MatrixVec,
    PerfRelative,
};

#[allow(dead_code)]
fn perf_test() {
    let mut l1 = ds_rs::linked_list::LinkedList::new();
    let mut l2 = std::collections::linked_list::LinkedList::new();
    let perf = PerfRelative::new("ds_rs", "std");

    perf.test(
        "push_back",
        10000,
        || {
            l1.push_back(42);
        },
        || {
            l2.push_back(42);
        },
    );

    perf.test(
        "pop_back",
        5000,
        || {
            let _ = l1.pop_back();
        },
        || {
            let _ = l2.pop_back();
        },
    );

    perf.test(
        "pop_front",
        5000,
        || {
            let _ = l1.pop_front();
        },
        || {
            let _ = l2.pop_front();
        },
    );

    perf.test(
        "push_front",
        1000,
        || {
            let _ = l1.push_front(68);
        },
        || {
            let _ = l2.push_front(68);
        },
    );

    let mut iter1 = l1.iter();
    let mut iter2 = l2.iter();
    perf.test(
        "iter",
        900,
        || {
            iter1.next();
        },
        || {
            iter2.next();
        },
    );

    perf.test(
        "clear",
        1,
        || {
            l1.clear();
        },
        || {
            l2.clear();
        },
    );
}

fn main() {
    let mut g = Graph::<char, u8, usize>::new();
    g.insert(graph::Node::new('A', 0));
    g.insert(graph::Node::new('B', 0));
    g.insert(graph::Node::new('C', 0));
    g.insert(graph::Node::new('D', 0));
    g.insert(graph::Node::new('F', 0));
    g.insert(graph::Node::new('G', 0));
    g.insert(graph::Node::new('H', 0));

    g.insert_edge('A', 'B', 4).unwrap();
    g.insert_edge('A', 'C', 4).unwrap();
    g.insert_edge('C', 'F', 2).unwrap();
    g.insert_edge('C', 'B', 3).unwrap();
    g.insert_edge('B', 'C', 5).unwrap();
    g.insert_edge('B', 'D', 7).unwrap();
    g.insert_edge('D', 'H', 6).unwrap();
    g.insert_edge('D', 'G', 3).unwrap();
    g.insert_edge('F', 'D', 1).unwrap();
    g.insert_edge('F', 'H', 7).unwrap();
    g.insert_edge('G', 'H', 1).unwrap();

    let inst = Instant::now();
    let res = g.dijkstra_shortest_path(&'A', &'H');
    dbg!(inst.elapsed());
    dbg!(res);

    let m1: MatrixVec<usize> = MatrixVec::from([[1, 2], [4, 5], [7, 8]]);
    let m2: MatrixVec<usize> = MatrixVec::from([[1, 2, 3, 0], [4, 5, 6, 7]]);

    dbg!(m1.mult(&m2));
}
