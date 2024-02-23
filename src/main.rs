use std::time::Instant;

use ds_rs::{binary_tree::BinaryTree, rand_vec_gen, PerfRelative};

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
    let mut v = rand_vec_gen(10000);
    let v_clone = v.clone();

    let b = BinaryTree::from(v_clone);

    // let perf = PerfRelative::new("stable", "binary_tree");

    let inst = Instant::now();
    v.sort();
    let elpsd1 = Instant::now() - inst;
    let inst = Instant::now();
    let x = b.into_sorted_vec();
    let elpsd2 = Instant::now() - inst;
    assert_eq!(v, x);
    dbg!(elpsd1);
    dbg!(elpsd2);
}
