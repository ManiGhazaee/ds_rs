use pretty_assertions::assert_eq;

#[test]
fn test_queue_operations() {
    let mut queue: ds_rs::queue::array::Queue<i32, 5> = ds_rs::queue::array::Queue::new();

    assert!(queue.is_empty());

    queue.enq(1);
    queue.enq(2);
    queue.enq(3);
    assert_eq!(queue.deq(), 1);
    assert_eq!(queue.deq(), 2);
    assert_eq!(queue.deq(), 3);
    assert!(queue.is_empty());

    for i in 0..5 {
        queue.enq(i);
    }
    assert!(queue.is_full());

    for i in 0..3 {
        assert_eq!(queue.deq(), i);
    }
    assert!(!queue.is_empty());

    queue.enq(5);
    queue.enq(6);
    assert_eq!(queue.deq(), 3);
    assert_eq!(queue.deq(), 4);
    assert_eq!(queue.deq(), 5);
    assert_eq!(queue.deq(), 6);
    assert!(queue.is_empty());

    for i in 0..5 {
        queue.enq(i);
    }
    queue.clear();
    assert!(queue.is_empty());

    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);

    queue.enq(10);
    queue.enq(20);
    assert_eq!(queue.front(), Some(&10));
    assert_eq!(queue.back(), Some(&20));
}

#[test]
fn test_iter() {
    let q: ds_rs::queue::array::Queue<i32, 10> = ds_rs::queue::array::Queue::new();
    let mut iter = q.iter();
    assert_eq!(iter.next(), None);

    let mut q: ds_rs::queue::array::Queue<i32, 10> = ds_rs::queue::array::Queue::new();
    q.enq(1);
    q.enq(2);
    q.enq(3);
    q.enq(4);
    q.enq(5);
    q.enq(6);
    let mut iter = q.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut() {
    let mut q: ds_rs::queue::array::Queue<i32, 10> = ds_rs::queue::array::Queue::new();
    q.enq(1);
    q.enq(2);
    q.enq(3);
    q.enq(4);
    q.enq(5);
    q.enq(6);
    for i in q.iter_mut() {
        if *i % 2 == 0 {
            *i = 0;
        }
    }
    let mut iter = q.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter() {
    let mut q: ds_rs::queue::array::Queue<i32, 10> = ds_rs::queue::array::Queue::new();
    q.enq(1);
    q.enq(2);
    q.enq(3);
    q.enq(4);
    q.enq(5);
    q.enq(6);
    let mut x = q.into_iter().map(|i| if i % 2 == 0 { 0 } else { i });
    assert_eq!(x.next(), Some(1));
    assert_eq!(x.next(), Some(0));
    assert_eq!(x.next(), Some(3));
    assert_eq!(x.next(), Some(0));
    assert_eq!(x.next(), Some(5));
    assert_eq!(x.next(), Some(0));
    assert_eq!(x.next(), None);
}

#[test]
fn test_queue_operations_ll() {
    let mut queue: ds_rs::queue::linked_list::Queue<i32> = ds_rs::queue::linked_list::Queue::new();

    assert!(queue.is_empty());

    queue.enq(1);
    queue.enq(2);
    queue.enq(3);
    assert_eq!(queue.deq().unwrap(), 1);
    assert_eq!(queue.deq().unwrap(), 2);
    assert_eq!(queue.deq().unwrap(), 3);
    assert!(queue.is_empty());

    for i in 0..5 {
        queue.enq(i);
    }

    for i in 0..3 {
        assert_eq!(queue.deq().unwrap(), i);
    }

    assert!(!queue.is_empty());

    queue.enq(5);
    queue.enq(6);
    assert_eq!(queue.deq().unwrap(), 3);
    assert_eq!(queue.deq().unwrap(), 4);
    assert_eq!(queue.deq().unwrap(), 5);
    assert_eq!(queue.deq().unwrap(), 6);
    assert!(queue.is_empty());

    for i in 0..5 {
        queue.enq(i);
    }

    queue.clear();
    assert!(queue.is_empty());

    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);

    queue.enq(10);
    queue.enq(20);
    assert_eq!(queue.front(), Some(&10));
    assert_eq!(queue.back(), Some(&20));
}

#[test]
fn test_iter_ll() {
    let q: ds_rs::queue::linked_list::Queue<i32> = ds_rs::queue::linked_list::Queue::new();
    let mut iter = q.iter();
    assert_eq!(iter.next(), None);

    let mut q: ds_rs::queue::linked_list::Queue<i32> = ds_rs::queue::linked_list::Queue::new();
    q.enq(1);
    q.enq(2);
    q.enq(3);
    q.enq(4);
    q.enq(5);
    q.enq(6);
    let mut iter = q.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_ll() {
    let mut q: ds_rs::queue::linked_list::Queue<i32> = ds_rs::queue::linked_list::Queue::new();
    q.enq(1);
    q.enq(2);
    q.enq(3);
    q.enq(4);
    q.enq(5);
    q.enq(6);
    for i in q.iter_mut() {
        if *i % 2 == 0 {
            *i = 0;
        }
    }
    let mut iter = q.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), None);
}
