#[test]
fn test_push_and_pop() {
    let mut stack = ds_rs::stack::array::Stack::<i32, 3>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), 3);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.pop(), 1);
}

#[test]
#[should_panic]
fn test_push_full() {
    let mut stack = ds_rs::stack::array::Stack::<i32, 3>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
}

#[test]
#[should_panic]
fn test_pop_empty() {
    let mut stack = ds_rs::stack::array::Stack::<i32, 3>::new();
    assert!(stack.is_empty());
    stack.pop();
}

#[test]
fn test_peek() {
    let mut stack = ds_rs::stack::array::Stack::<i32, 3>::new();
    stack.push(42);
    stack.push(2);
    assert_eq!(*stack.peek(), 2);
    stack.push(5);
    assert_eq!(*stack.peek(), 5);
    *stack.peek_mut() = 6;
    assert_eq!(*stack.peek(), 6);
}

#[test]
fn test_push_and_pop_ll() {
    let mut stack = ds_rs::stack::linked_list::Stack::<i32>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop().unwrap(), 3);
    assert_eq!(stack.pop().unwrap(), 2);
    assert_eq!(stack.pop().unwrap(), 1);
}

#[test]
#[should_panic]
fn test_pop_empty_ll() {
    let mut stack = ds_rs::stack::linked_list::Stack::<i32>::new();
    assert!(stack.is_empty());
    stack.pop().unwrap();
}

#[test]
fn test_peek_ll() {
    let mut stack = ds_rs::stack::linked_list::Stack::<i32>::new();
    stack.push(42);
    stack.push(2);
    assert_eq!(*stack.peek().unwrap(), 2);
    stack.push(5);
    assert_eq!(*stack.peek().unwrap(), 5);
    *stack.peek_mut().unwrap() = 6;
    assert_eq!(*stack.peek().unwrap(), 6);
}
