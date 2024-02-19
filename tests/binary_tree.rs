use ds_rs::binary_tree::BinaryTree;

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