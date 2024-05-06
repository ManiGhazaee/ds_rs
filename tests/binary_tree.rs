#![cfg(test)]
use std::{cmp::Ordering, rc::Rc};

use ds_rs::binary_tree::{BinaryTree, Node};

#[test]
fn test_basic() {
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

    assert_eq!(r.right().parent().val(), r.val());
    assert_eq!(r.left().parent().val(), r.val());
    assert_eq!(r.left().left().parent().val(), r.left().val());
    assert_eq!(r.right().left().parent().val(), r.right().val());
}

#[test]
#[should_panic]
fn test_parent_panic_empty() {
    let b: BinaryTree<isize> = BinaryTree::new();
    b.root().parent();
}

#[test]
#[should_panic]
fn test_parent_panic() {
    let mut b: BinaryTree<isize> = BinaryTree::new();
    b.push(2);
    b.root().parent();
}

#[test]
fn test_clear() {
    let mut b: BinaryTree<isize> = BinaryTree::new();
    b.push(1);
    b.push(2);
    b.push(3);
    b.push(4);
    b.push(5);
    b.push(6);
    b.push(7);

    let x = b.root().left().left().val_clone();
    let y = b.root().left().left().val();

    assert!(!b.is_empty());
    b.clear();
    assert!(b.is_empty());
    assert_eq!(x, Some(4));
    assert_eq!(y.unwrap().as_ref(), &4);
    assert_eq!(b.root().val(), None);
}

#[test]
fn test_is_root() {
    let mut b: BinaryTree<isize> = BinaryTree::new();
    b.push(1);
    b.push(2);
    b.push(3);
    b.push(4);
    b.push(5);
    b.push(6);
    b.push(7);

    let x = b.root();
    let y = b.root().left();
    let z = b.root().right().right();

    assert!(x.is_root());
    assert!(!y.is_root());
    assert!(!z.is_root());
}

#[test]
fn test_set() {
    let mut b: BinaryTree<isize> = BinaryTree::new();
    b.push(1);
    b.push(2);
    b.push(3);
    b.push(4);
    b.push(5);
    b.push(6);
    b.push(7);

    assert_eq!(b.len(), 7);

    let x = b.root();
    let y = b.root().set_left(b.root().left().val_clone().unwrap() * 2);
    let z = b
        .root()
        .set_right(b.root().right().val_clone().unwrap() * 3);

    assert_eq!(b.len(), 7);
    assert_eq!(x.val_clone(), Some(1));
    assert_eq!(y.val_clone(), Some(4));
    assert_eq!(z.val_clone(), Some(9));
    assert_eq!(x.left().val_clone(), Some(4));
    assert_eq!(x.right().val_clone(), Some(9));
}

#[test]
fn test_heapify() {
    let mut b = BinaryTree::new();
    b.push(9);
    b.push(8);
    b.push(7);
    b.push(6);
    b.push(4);
    b.push(3);
    b.push(2);

    assert!(!is_min_heap(&b.as_vec()));
    assert!(!b.is_min_heap());

    b.heapify_min();

    assert!(is_min_heap(&b.as_vec()));
    assert!(b.is_min_heap());
    assert!(!is_max_heap(&b.as_vec()));

    b.clear();

    b.push(1);
    b.push(2);
    b.push(5);
    b.push(2);
    b.push(5);
    b.push(9);
    b.push(7);

    assert!(!is_max_heap(&b.as_vec()));
    assert!(!b.is_max_heap());

    b.heapify_max();

    assert!(is_max_heap(&b.as_vec()));
    assert!(b.is_max_heap());
    assert!(!is_min_heap(&b.as_vec()));
}

#[test]
fn test_as_vec() {
    let b = BinaryTree::new();

    b.set_root(0).set_left(1).set_right(2);
    b.root().set_right(3).set_right(4);
    // [0, 1, 3, None, 2, None, 4]

    let vec = b.as_vec();
    let vec_raw = b.as_vec_raw();

    assert_eq!(vec[0].val_clone(), Some(0));
    assert_eq!(vec[1].val_clone(), Some(1));
    assert_eq!(vec[2].val_clone(), Some(3));
    assert_eq!(vec[3].val_clone(), None);
    assert_eq!(vec[4].val_clone(), Some(2));
    assert_eq!(vec[5].val_clone(), None);
    assert_eq!(vec[6].val_clone(), Some(4));

    assert_eq!(vec_raw[0], Some(Rc::new(0)));
    assert_eq!(vec_raw[1], Some(Rc::new(1)));
    assert_eq!(vec_raw[2], Some(Rc::new(3)));
    assert_eq!(vec_raw[3], None);
    assert_eq!(vec_raw[4], Some(Rc::new(2)));
    assert_eq!(vec_raw[5], None);
    assert_eq!(vec_raw[6], Some(Rc::new(4)));
}

#[test]
fn test_capacity() {
    let b = BinaryTree::with_capacity(100);
    b.set_root(0);
    assert!(b.capacity() >= 100);
}

#[test]
fn test_empty() {
    let mut b: BinaryTree<i32> = BinaryTree::new();
    assert!(b.as_vec().is_empty());
    assert!(b.as_vec_raw().is_empty());
    b.heapify_max();
    b.heapify_min();
    b.is_heap_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(b.is_empty());
    assert!(b.is_max_heap());
    assert!(b.is_min_heap());
    assert_eq!(b.len(), 0);
    b.pop();
    assert_eq!(b.root().val(), None);
    assert_eq!(b.root().val_clone(), None);
}

#[test]
fn test_sorted() {
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

    let vec = b.into_sorted_vec();

    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let b = BinaryTree::new();

    b.set_root(0).set_left(1).set_right(2);
    b.root().set_right(3).set_right(4);
    // [0, 1, 3, None, 2, None, 4]

    let vec = b.into_sorted_vec();

    assert_eq!(vec, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_sorted_empty() {
    let b: BinaryTree<i32> = BinaryTree::new();

    let vec = b.into_sorted_vec();

    assert_eq!(vec, vec![])
}

#[test]
fn test_into_vec() {
    let b = BinaryTree::new();

    b.set_root(0).set_left(1).set_right(2);
    b.root().set_right(3).set_right(4);
    // [0, 1, 3, None, 2, None, 4]
    let v = b.into_vec();
    assert_eq!(v, vec![0, 1, 3, 2, 4]);
}

pub fn is_heap_by<T, F>(vec: &[Node<T>], compare: F) -> bool
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in (0..vec.len()).rev() {
        let parent = if !vec[i].is_root() {
            vec[i].parent()
        } else {
            continue;
        };
        let vals = if let (Some(val), Some(pval)) = (vec[i].val(), parent.val()) {
            (val, pval)
        } else {
            continue;
        };
        if let Ordering::Less = compare(&*vals.0, &*vals.1) {
            return false;
        }
    }
    true
}

pub fn is_max_heap<T: PartialOrd>(vec: &Vec<Node<T>>) -> bool {
    is_heap_by(&vec[..], |a, b| b.partial_cmp(a).unwrap())
}

pub fn is_min_heap<T: PartialOrd>(vec: &Vec<Node<T>>) -> bool {
    is_heap_by(&vec[..], |a, b| a.partial_cmp(b).unwrap())
}

#[test]
fn test_basic_rawptr() {
    let mut b: ds_rs::binary_tree::rawptr::BinaryTree<char> =
        ds_rs::binary_tree::rawptr::BinaryTree::new();
    assert!(b.is_empty());

    assert_eq!(b.root(), None);

    {
        let r = b.set_root_mut('1');

        assert_eq!(r.left(), None);
        assert_eq!(r.right(), None);

        *r.val_mut() = '2';

        r.set_left('3');
        r.set_right('4');
    }

    assert_eq!(b.root().unwrap().left().unwrap().val(), &'3');
    assert_eq!(b.root().unwrap().right().unwrap().val(), &'4');

    {
        let r = b.root_mut().unwrap();
        r.left_mut().unwrap().set_left('7');
    }

    let ll = b.root().unwrap().left().unwrap().left().unwrap().val();

    assert_eq!(ll, &'7');

    let r = b.root_mut().unwrap();
    r.right_mut().unwrap().set_right('8');
    let rr = r.right().unwrap().right().unwrap().val();
    assert_eq!(rr, &'8');

    assert_eq!(r.right().unwrap().parent().unwrap().val(), r.val());
    assert_eq!(r.left().unwrap().parent().unwrap().val(), r.val());
    assert_eq!(
        r.left().unwrap().left().unwrap().parent().unwrap().val(),
        r.left().unwrap().val()
    );

    b.clear();

    assert!(b.is_empty());
}
