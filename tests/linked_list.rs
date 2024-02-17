use ds_rs::linked_list::LinkedList;

#[test]
fn test_linked_list_soft() {
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
fn test_empty() {
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
fn test_pop_back() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.pop_back();
    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(1));
    assert_eq!(list.back(), Some(1));
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

#[test]
fn test_get() {
    let mut l = LinkedList::new();
    l.push_back('a');
    l.push_back('b');
    l.push_back('c');

    assert_eq!(l.get(0), Some('a'));
    assert_eq!(l.get(1), Some('b'));
    assert_eq!(l.get(2), Some('c'));
}

#[test]
fn test_linked_list_hard() {
    let mut l = LinkedList::new();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    l.push_front("1");

    assert!(!l.is_empty());
    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some("1"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some("1"));
    assert_eq!(l.back(), Some("1"));

    l.pop_back();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    l.push_front("1");
    l.pop_front();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    l.push_back("1");

    assert!(!l.is_empty());
    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some("1"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some("1"));
    assert_eq!(l.back(), Some("1"));

    l.pop_back();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    l.push_back("1");
    l.pop_front();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    // testing len = 2 pop
    l.push_back("1");
    l.push_back("2");

    assert!(!l.is_empty());
    assert_eq!(l.len(), 2);
    assert_eq!(l.get(0), Some("1"));
    assert_eq!(l.get(1), Some("2"));
    assert_eq!(l.front(), Some("1"));
    assert_eq!(l.back(), Some("2"));

    l.pop_back();

    assert!(!l.is_empty());
    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some("1"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some("1"));
    assert_eq!(l.back(), Some("1"));

    l.clear();
    assert!(l.is_empty());

    l.push_back("1");
    l.push_back("2");
    l.pop_front();

    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some("2"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some("2"));
    assert_eq!(l.back(), Some("2"));

    l.clear();

    l.push_back("2");
    l.push_front("1");
    l.push_back("3");
    l.push_front("0");
    l.push_back("4");

    assert_eq!(l.get(0), Some("0"));
    assert_eq!(l.get(1), Some("1"));
    assert_eq!(l.get(2), Some("2"));
    assert_eq!(l.get(3), Some("3"));
    assert_eq!(l.get(4), Some("4"));
    assert_eq!(l.get(5), None);

    l.insert(0, "x");
    l.insert(4, "y");
    l.insert(5, "z");

    assert_eq!(l.get(0), Some("x"));
    assert_eq!(l.get(1), Some("0"));
    assert_eq!(l.get(2), Some("1"));
    assert_eq!(l.get(3), Some("2"));
    assert_eq!(l.get(4), Some("y"));
    assert_eq!(l.get(5), Some("z"));
    assert_eq!(l.get(6), Some("3"));
    assert_eq!(l.get(7), Some("4"));
    assert_eq!(l.get(8), None);
    assert_eq!(l.front(), Some("x"));
    assert_eq!(l.back(), Some("4"));

    l.clear();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    let mut l2 = LinkedList::new();
    l2.push_back("1");
    l2.push_back("2");
    l.append(&mut l2);

    assert_eq!(l2.len(), 0);
    assert_eq!(l.len(), 2);
    assert_eq!(l2.get(0), None);
    assert_eq!(l2.get(1), None);
    assert_eq!(l.get(0), Some("1"));
    assert_eq!(l.get(1), Some("2"));
    assert_eq!(l2.front(), None);
    assert_eq!(l2.back(), None);
    assert_eq!(l.front(), Some("1"));
    assert_eq!(l.back(), Some("2"));
}
