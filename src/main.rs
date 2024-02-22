use std::time::Instant;

use ds_rs::{binary_tree::BinaryTree, PerfRelative};

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
    let mut v = Vec::new();

    v.push(9);
    v.push(7);
    v.push(8);
    v.push(3);
    v.push(1);
    v.push(5);
    v.push(6);
    v.push(4);
    v.push(2);

   let mut b = BinaryTree::new();

    b.push(9);
    b.push(7);
    b.push(8);
    b.push(3);
    b.push(1);
    b.push(5);
    b.push(6);
    b.push(4);
    b.push(2);

    let inst = Instant::now();
    let _ = b.into_sorted_vec();
    let elpsd1 = inst.elapsed().as_micros();
    let inst = Instant::now();
    v.sort();
    let elpsd2 = inst.elapsed().as_micros();

    dbg!(elpsd1);
    dbg!(elpsd2);


}
