use ds_rs::stack::Stack;

#[test]
fn test_push_and_pop() {
    let mut stack = Stack::<i32, 3>::new();
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
    let mut stack = Stack::<i32, 3>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4); // Stack is full, should panic
}

#[test]
#[should_panic]
fn test_pop_empty() {
    let mut stack = Stack::<i32, 3>::new();
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), 0); // Popping from an empty stack should return default value
}

#[test]
fn test_peek() {
    let mut stack = Stack::<i32, 3>::new();
    stack.push(42);
    stack.push(2);
    assert_eq!(*stack.peek(), 2);
}
