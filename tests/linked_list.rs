use ds_rs::LinkedList;

#[test]
fn test_linked_list() {
    let mut list: LinkedList<i32> = LinkedList::new();

    assert_eq!(list.is_empty(), true);
    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
    assert_eq!(list.back(), None);

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.is_empty(), false);
    assert_eq!(list.len(), 3);
    assert_eq!(list.front(), Some(1));
    assert_eq!(list.back(), Some(3));
    assert_eq!(list.get(0), Some(1));

    list.push_front(0);

    assert_eq!(list.len(), 4);
    assert_eq!(list.front(), Some(0));

    list.insert(2, 10);

    assert_eq!(list.len(), 5);
    assert_eq!(list.front(), Some(0));
    assert_eq!(list.back(), Some(3));

    assert_eq!(list.get(0), Some(0));
    assert_eq!(list.get(2), Some(10));
    assert_eq!(list.get(4), Some(3));
}

#[test]
fn test_new_linked_list_is_empty() {
    let list: LinkedList<i32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
    assert_eq!(list.back(), None);
    assert_eq!(list.get(0), None);
}

#[test]
fn test_push_back() {
    let mut list = LinkedList::new();
    list.push_back(1);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(1));
    assert_eq!(list.back(), Some(1));
    assert_eq!(list.get(0), Some(1));
}

#[test]
fn test_push_front() {
    let mut list = LinkedList::new();
    list.push_front(1);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(1));
    assert_eq!(list.back(), Some(1));
    assert_eq!(list.get(0), Some(1));
}

#[test]
fn test_pop_front() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.pop_front();
    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(2));
    assert_eq!(list.back(), Some(2));
}

#[test]
fn test_insert() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(3);
    list.insert(1, 2);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get(1), Some(2));
}

#[test]
fn test_append() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');

    list1.append(&mut list2);

    assert_eq!(list1.get(0), Some('a'));
    assert_eq!(list1.get(1), Some('b'));
    assert_eq!(list1.get(2), Some('c'));

    assert!(list2.is_empty());
}
