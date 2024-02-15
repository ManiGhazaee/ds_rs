use ds_rs::LinkedList;

#[test]
fn test_linked_list() {
    let mut list: LinkedList<i32> = LinkedList::new();

    assert_eq!(list.is_empty(), true);
    assert_eq!(list.size(), 0);
    assert_eq!(list.head(), None);
    assert_eq!(list.tail(), None);

    list.add(1);
    list.add(2);
    list.add(3);

    assert_eq!(list.is_empty(), false);
    assert_eq!(list.size(), 3);
    assert_eq!(list.head(), Some(1));
    assert_eq!(list.tail(), Some(3));

    list.push(0);

    assert_eq!(list.size(), 4);
    assert_eq!(list.head(), Some(0));

    list.insert(2, 10).unwrap();

    assert_eq!(list.size(), 5);
    assert_eq!(list.head(), Some(0));
    assert_eq!(list.tail(), Some(3));

    // Test retrieving elements by index
    assert_eq!(list.get(0), Some(0));
    assert_eq!(list.get(2), Some(10));
    assert_eq!(list.get(4), Some(3));
}
