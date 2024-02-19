use ds_rs::binary_tree::BinaryTree;

#[test]
fn test_basic() {
    let mut b = BinaryTree::new();
    assert_eq!(b.len(), 0);

    b.push('1');
    assert_eq!(b.len(), 1);

    let mut r = b.root_node();

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
