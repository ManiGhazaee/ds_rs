# Data Structures in Rust

## Linked List

[source](./src/linked_list.rs)

### `LinkedList<T>`:

| Method | Time Complexity |
| ------ | --------------- |
| `new() -> Self`| *O*(1) 
| `is_empty() -> bool`| *O*(1) 
| `len() -> usize`| *O*(1) 
| `back() -> Option<T>`| *O*(1) 
| `front() -> Option<T>`| *O*(1) 
| `push_back(val: T)`| *O*(1) 
| `push_front(val: T)`| *O*(1) 
| `pop_back() -> Option<T>`| *O*(1) 
| `pop_front() -> Option<T>`| *O*(1) 
| `append(other: &mut LinkedList<T>)`| *O*(1) 
| `clear()`| *O*(n) 
| `insert(index: usize, val: T)`| *O*(n) 
| `get(index: usize) -> Option<T>`| *O*(n) 
| `remove(index: usize) -> Option<T>`| *O*(n) 
| `change(index: usize)`| *O*(n) 

## Binary Tree

### `BinaryTree<T>`:

| Method | Time Complexity |
| ------ | --------------- |
| `new() -> Self`| *O*(1) 
| `with_capacity(capacity: usize) -> Self`| *O*(1)?
| `is_empty() -> bool`| *O*(1) 
| `len() -> usize`| *O*(1) 
| `capacity() -> usize`| *O*(1) 
| `push(val: T)`| *O*(1) 
| `pop()`| *O*(1) 
| `root() -> Node<T>`| *O*(1) 
| `set_root(val: T) -> Node<T>`| *O*(1) 
| `clear()`| *O*(1)? 
| `as_vec() -> Vec<Node<T>>`| *O*(n) 
| `as_vec_raw() -> Vec<Option<Rc<T>>>`| *O*(n) 
| `is_heap_by<F: Fn(&T, &T) -> Ordering>(compare: F) -> bool`| *O*(n) 
| `is_max_heap() -> bool`| *O*(n) 
| `is_min_heap() -> bool`| *O*(n) 
| `heapify_by<F: Fn(&T, &T) -> Ordering>(compare: F)`| *O*(n log(n)) 
| `heapify_min()`| *O*(n log(n)) 
| `heapify_max()`| *O*(n log(n)) 
| `into_sorted_vec_by<F: Fn(&T, &T) -> Ordering>(compare: F) -> Vec<T>`| *O*(n log(n)) 
| `into_sorted_vec() -> Vec<T>`| *O*(n log(n)) 
| `into_vec() -> Vec<T>`| *O*(n) 
| `from(value: [T; N]) -> Self`| *O*(n) 
| `from(value: &[T]) -> Self`| *O*(n) 
| `from(value: Vec<T>) -> Self`| *O*(n) 


### `Node<T>`:

| Method | Time Complexity |
| ------ | --------------- |
| `left() -> Node<T>`| *O*(1) 
| `right() -> Node<T>`| *O*(1) 
| `parent() -> Node<T>`| *O*(1) 
| `val() -> Option<Rc<T>>`| *O*(1) 
| `change(new_val: T)`| *O*(1) 
| `is_root() -> bool`| *O*(1) 
| `val_clone() -> Option<T>`| *O*(1) 
| `set_left(val: T) -> Self`| *O*(1) 
| `set_right(val: T) -> Self`| *O*(1) 

## Queue

### `Queue<T>`:

| Method | Time Complexity |
| ------ | --------------- |
| `new(capacity: usize) -> Self`| *O*(1) 
| `is_empty() -> bool`| *O*(1) 
| `is_full() -> bool`| *O*(1) 
| `size() -> usize`| *O*(1) 
| `capacity() -> usize`| *O*(1) 
| `tail() -> Option<&T>`| *O*(1) 
| `head() -> Option<&T>`| *O*(1) 
| `enq() -> Result<(), ()>`| *O*(n) 
| `deq() -> Result<(), ()>`| *O*(1) 
| `deq_all()`| *O*(n) 
