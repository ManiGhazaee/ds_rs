// use ds_rs::queue::Queue;

// #[test]
// fn queue_test() {
//     let mut q: Queue<&str> = Queue::new(10);
//     q.enq("v1").unwrap();
//     assert_eq!(q.is_empty(), false);
//     assert_eq!(q.is_full(), false);
//     assert_eq!(q.capacity(), 10);
//     assert_eq!(q.size(), 1);
//     assert_eq!(q.tail().unwrap(), &"v1");
//     assert_eq!(q.head().unwrap(), &"v1");
//     q.enq("v2").unwrap();
//     assert_eq!(q.is_empty(), false);
//     assert_eq!(q.is_full(), false);
//     assert_eq!(q.capacity(), 10);
//     assert_eq!(q.size(), 2);
//     assert_eq!(q.tail().unwrap(), &"v1");
//     assert_eq!(q.head().unwrap(), &"v2");
//     q.deq().unwrap();
//     assert_eq!(q.is_empty(), false);
//     assert_eq!(q.is_full(), false);
//     assert_eq!(q.capacity(), 10);
//     assert_eq!(q.size(), 1);
//     assert_eq!(q.tail().unwrap(), &"v2");
//     assert_eq!(q.head().unwrap(), &"v2");
//     q.deq().unwrap();
//     assert_eq!(q.deq(), Err(()));
//     assert_eq!(q.is_empty(), true);
//     assert_eq!(q.is_full(), false);
//     assert_eq!(q.capacity(), 10);
//     assert_eq!(q.size(), 0);
//     assert_eq!(q.tail(), None);
//     assert_eq!(q.head(), None);
//     q.enq("1").unwrap();
//     q.enq("2").unwrap();
//     q.enq("3").unwrap();
//     q.enq("4").unwrap();
//     q.enq("5").unwrap();
//     q.enq("6").unwrap();
//     q.enq("7").unwrap();
//     q.enq("8").unwrap();
//     q.enq("9").unwrap();
//     q.enq("10").unwrap();
//     assert_eq!(q.enq("11"), Err(()));
//     assert_eq!(q.is_empty(), false);
//     assert_eq!(q.is_full(), true);
//     assert_eq!(q.capacity(), 10);
//     assert_eq!(q.size(), 10);
//     assert_eq!(q.tail().unwrap(), &"1");
//     assert_eq!(q.head().unwrap(), &"10");
//     q.deq_all();
//     assert_eq!(q.is_empty(), true);
//     assert_eq!(q.is_full(), false);
//     assert_eq!(q.capacity(), 10);
//     assert_eq!(q.size(), 0);
//     assert_eq!(q.tail(), None);
//     assert_eq!(q.head(), None);
// }

use ds_rs::queue::Queue;

#[test]
fn test_queue_operations() {
    let mut queue: Queue<i32, 5> = Queue::new();

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
    dbg!(&queue);

    queue.enq(5);
    queue.enq(6);
    dbg!(&queue);
    assert_eq!(queue.deq(), 3);
    assert_eq!(queue.deq(), 4);
    assert_eq!(queue.deq(), 5);
    assert_eq!(queue.deq(), 6);
    assert!(queue.is_empty());
    dbg!(&queue);

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
