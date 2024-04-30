use ds_rs::linked_list::LinkedList;

#[test]
fn test_basic() {
    let mut m = LinkedList::<Box<_>>::new();
    assert_eq!(m.pop_front(), None);
    assert_eq!(m.pop_back(), None);
    assert_eq!(m.pop_front(), None);
    m.push_front(Box::new(1));
    assert_eq!(m.pop_front(), Some(Box::new(1)));
    m.push_back(Box::new(2));
    m.push_back(Box::new(3));
    assert_eq!(m.len(), 2);
    assert_eq!(m.pop_front(), Some(Box::new(2)));
    assert_eq!(m.pop_front(), Some(Box::new(3)));
    assert_eq!(m.len(), 0);
    assert_eq!(m.pop_front(), None);
    m.push_back(Box::new(1));
    m.push_back(Box::new(3));
    m.push_back(Box::new(5));
    m.push_back(Box::new(7));
    assert_eq!(m.pop_front(), Some(Box::new(1)));

    let mut l: LinkedList<i32> = LinkedList::new();
    assert_eq!(l.is_empty(), true);
    assert_eq!(l.len(), 0);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    assert_eq!(l.is_empty(), false);
    assert_eq!(l.len(), 3);
    assert_eq!(l.front(), Some(1));
    assert_eq!(l.back(), Some(3));
    assert_eq!(l.get(0), Some(1));
    l.push_front(0);
    assert_eq!(l.len(), 4);
    assert_eq!(l.front(), Some(0));
    l.insert(2, 10);
    assert_eq!(l.len(), 5);
    assert_eq!(l.front(), Some(0));
    assert_eq!(l.back(), Some(3));
    assert_eq!(l.get(0), Some(0));
    assert_eq!(l.get(2), Some(10));
    assert_eq!(l.get(4), Some(3));
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
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.len(), 1);
    assert_eq!(list.front(), Some(2));
    assert_eq!(list.back(), Some(2));
}

#[test]
fn test_pop_back() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.pop_back(), Some(2));
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
    let mut l = LinkedList::new();
    l.push_back('a');
    let mut l2 = LinkedList::new();
    l2.push_back('b');
    l2.push_back('c');

    l.append(&mut l2);

    assert_eq!(l.get(0), Some('a'));
    assert_eq!(l.get(1), Some('b'));
    assert_eq!(l.get(2), Some('c'));

    assert!(l2.is_empty());

    l.clear();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains('1'));

    let mut l2 = LinkedList::new();
    l2.push_back('1');
    l2.push_back('2');
    l.append(&mut l2);

    assert_eq!(l2.len(), 0);
    assert_eq!(l.len(), 2);
    assert_eq!(l2.get(0), None);
    assert_eq!(l2.get(1), None);
    assert_eq!(l.get(0), Some('1'));
    assert_eq!(l.get(1), Some('2'));
    assert_eq!(l2.front(), None);
    assert_eq!(l2.back(), None);
    assert_eq!(l.front(), Some('1'));
    assert_eq!(l.back(), Some('2'));
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
fn test_push_pop_insert_get() {
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

    assert_eq!(l.pop_back(), Some("1"));

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    l.push_front("1");
    assert_eq!(l.pop_front(), Some("1"));

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

    assert_eq!(l.pop_back(), Some("1"));

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    assert!(!l.contains("1"));

    l.push_back("1");
    assert_eq!(l.pop_front(), Some("1"));

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

    assert_eq!(l.pop_back(), Some("2"));

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
    assert_eq!(l.pop_front(), Some("1"));

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
}

#[test]
fn test_iter() {
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(0);
    list.push_back(1);
    list.push_back(2);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_remove() {
    let mut l = LinkedList::new();

    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    l.push_back(5);
    l.push_back(6);
    l.push_back(7);
    l.push_back(8);

    assert_eq!(l.len(), 8);

    assert_eq!(l.remove(0), Some(1));
    assert_eq!(l.len(), 7);
    assert_eq!(l.get(0), Some(2));
    assert_eq!(l.get(1), Some(3));
    assert_eq!(l.get(2), Some(4));
    assert_eq!(l.get(3), Some(5));
    assert_eq!(l.get(4), Some(6));
    assert_eq!(l.get(5), Some(7));
    assert_eq!(l.get(6), Some(8));
    assert_eq!(l.front(), Some(2));
    assert_eq!(l.back(), Some(8));

    assert_eq!(l.remove(6), Some(8));
    assert_eq!(l.len(), 6);
    assert_eq!(l.get(0), Some(2));
    assert_eq!(l.get(1), Some(3));
    assert_eq!(l.get(2), Some(4));
    assert_eq!(l.get(3), Some(5));
    assert_eq!(l.get(4), Some(6));
    assert_eq!(l.get(5), Some(7));
    assert_eq!(l.front(), Some(2));
    assert_eq!(l.back(), Some(7));

    assert_eq!(l.remove(1), Some(3));
    assert_eq!(l.len(), 5);
    assert_eq!(l.get(0), Some(2));
    assert_eq!(l.get(1), Some(4));
    assert_eq!(l.get(2), Some(5));
    assert_eq!(l.get(3), Some(6));
    assert_eq!(l.get(4), Some(7));
    assert_eq!(l.front(), Some(2));
    assert_eq!(l.back(), Some(7));

    assert_eq!(l.remove(3), Some(6));
    assert_eq!(l.len(), 4);
    assert_eq!(l.get(0), Some(2));
    assert_eq!(l.get(1), Some(4));
    assert_eq!(l.get(2), Some(5));
    assert_eq!(l.get(3), Some(7));
    assert_eq!(l.front(), Some(2));
    assert_eq!(l.back(), Some(7));
}

#[test]
fn test_change() {
    let mut l = LinkedList::new();

    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);

    l.change(0, 4);
    l.change(1, 3);
    l.change(2, 2);
    l.change(3, 1);

    assert_eq!(l.get(0), Some(4));
    assert_eq!(l.get(1), Some(3));
    assert_eq!(l.get(2), Some(2));
    assert_eq!(l.get(3), Some(1));
    assert_eq!(l.front(), Some(4));
    assert_eq!(l.back(), Some(1));
    assert_eq!(l.len(), 4);
}

#[test]
#[should_panic]
fn test_remove_panic() {
    let mut l = LinkedList::new();
    l.push_back(1);
    l.remove(1);
}

#[test]
#[should_panic]
fn test_insert_panic() {
    let mut l = LinkedList::new();
    l.push_back(1);
    l.insert(2, 0);
}

#[test]
#[should_panic]
fn test_change_panic() {
    let mut l = LinkedList::new();
    l.push_back(1);
    l.change(2, 0);
}

#[test]
fn test_push_pop_insert_get_rawptr() {
    let mut l = ds_rs::linked_list::rawptr::LinkedList::new();

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    // assert!(!l.contains("1"));

    l.push_front("1");

    assert!(!l.is_empty());
    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some(&"1"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some(&"1"));
    assert_eq!(l.back(), Some(&"1"));

    assert_eq!(l.pop_back(), Some("1"));

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    // assert!(!l.contains("1"));

    l.push_front("1");
    assert_eq!(l.pop_front(), Some("1"));

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    // assert!(!l.contains("1"));

    l.push_back("1");

    assert!(!l.is_empty());
    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some(&"1"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some(&"1"));
    assert_eq!(l.back(), Some(&"1"));

    assert_eq!(l.pop_back(), Some("1"));

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    // assert!(!l.contains("1"));

    l.push_back("1");
    assert_eq!(l.pop_front(), Some("1"));

    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
    assert_eq!(l.get(0), None);
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), None);
    assert_eq!(l.back(), None);
    // assert!(!l.contains("1"));

    // testing len = 2 pop
    l.push_back("1");
    l.push_back("2");

    assert!(!l.is_empty());
    assert_eq!(l.len(), 2);
    assert_eq!(l.get(0), Some(&"1"));
    assert_eq!(l.get(1), Some(&"2"));
    assert_eq!(l.front(), Some(&"1"));
    assert_eq!(l.back(), Some(&"2"));

    assert_eq!(l.pop_back(), Some("2"));

    assert!(!l.is_empty());
    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some(&"1"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some(&"1"));
    assert_eq!(l.back(), Some(&"1"));

    l.clear();
    assert!(l.is_empty());

    l.push_back("1");
    l.push_back("2");
    assert_eq!(l.pop_front(), Some("1"));

    assert_eq!(l.len(), 1);
    assert_eq!(l.get(0), Some(&"2"));
    assert_eq!(l.get(1), None);
    assert_eq!(l.front(), Some(&"2"));
    assert_eq!(l.back(), Some(&"2"));

    l.clear();

    l.push_back("2");
    l.push_front("1");
    l.push_back("3");
    l.push_front("0");
    l.push_back("4");

    assert_eq!(l.get(0), Some(&"0"));
    assert_eq!(l.get(1), Some(&"1"));
    assert_eq!(l.get(2), Some(&"2"));
    assert_eq!(l.get(3), Some(&"3"));
    assert_eq!(l.get(4), Some(&"4"));
    assert_eq!(l.get(5), None);

    // l.insert(0, "x");
    // l.insert(4, "y");
    // l.insert(5, "z");

    // assert_eq!(l.get(0), Some(&"x"));
    // assert_eq!(l.get(1), Some(&"0"));
    // assert_eq!(l.get(2), Some(&"1"));
    // assert_eq!(l.get(3), Some(&"2"));
    // assert_eq!(l.get(4), Some(&"y"));
    // assert_eq!(l.get(5), Some(&"z"));
    // assert_eq!(l.get(6), Some(&"3"));
    // assert_eq!(l.get(7), Some(&"4"));
    // assert_eq!(l.get(8), None);
    // assert_eq!(l.front(), Some(&"x"));
    // assert_eq!(l.back(), Some(&"4"));
}
