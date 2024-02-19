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
    let mut b = BinaryTree::new();
    assert_eq!(b.len(), 0);

    b.push('1');
    assert_eq!(b.len(), 1);

    let mut r = b.root();

    assert_eq!(r.left().val(), None);
    assert_eq!(r.right().val(), None);

    r.change('2');

    b.push('3');
    b.push('4');

    let left = r.left().val().unwrap();
    let left = left.as_ref();

    let right = r.right().val().unwrap();
    let right = right.as_ref();

    r.left().change('5');
    r.right().change('6');

    assert_eq!(left, &'3');
    assert_eq!(right, &'4');
    assert_eq!(b.len(), 3);

    let left = r.left().val().unwrap();
    let left = left.as_ref();

    let right = r.right().val().unwrap();
    let right = right.as_ref();

    assert_eq!(left, &'5');
    assert_eq!(right, &'6');
    assert_eq!(b.len(), 3);

    let left = r.left().val_clone();
    let right = r.right().val_clone();

    assert_eq!(left, Some('5'));
    assert_eq!(right, Some('6'));
    assert_eq!(b.len(), 3);

    r.left().set_left('7');

    let left_left = r.left().left().val_clone();

    assert_eq!(left_left, Some('7'));
    assert_eq!(b.len(), 4);

    r.right().set_right('8');

    let right_right = r.right().right().val_clone();

    assert_eq!(right_right, Some('8'));
    assert_eq!(b.len(), 5);
}
