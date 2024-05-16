# Data Structures in Rust

This library is primarily for my own **educational purposes**. The implementations provided may **not** be the most efficient or best practices for production use. If you intend to use any part of this library, reviewing the **source code** is recommended.

## Available Data Structures 

- **LinkedList**
    - with raw pointers [`ds_rs::linked_list::rawptr`](./src/linked_list/rawptr.rs)
    - with `Cell` & `RefCell` [`ds_rs::linked_list::cell`](./src/linked_list/cell.rs)
- **Graph**
    - with `HashMap` [`ds_rs::graph::hash_map`](./src/graph/hash_map.rs)
    - with adjacency matrix [`ds_rs::graph::matrix`](./src/graph/matrix.rs)
- **Tree**
    - BinrayTree with raw pointers [`ds_rs::tree::rawptr`](./src/tree/rawptr.rs)
    - BinrayTree with `Cell` & `RefCell` [`ds_rs::tree::cell`](./src/tree/cell.rs)
- **Matrix**
    - with array [`ds_rs::matrix::array`](./src/matrix/array.rs)
    - with `Vec` [`ds_rs::matrix::vec`](./src/matrix/vec.rs)
- **Queue**
    - with array [`ds_rs::queue::array`](./src/queue/array.rs)
    - with linked list [`ds_rs::queue::linked_list`](./src/queue/linked_list.rs)
- **Stack**
    - with array [`ds_rs::stack::array`](./src/stack/array.rs)
    - with linked list [`ds_rs::stack::linked_list`](./src/stack/linked_list.rs)
- **BigInt** 
    - with `Vec` [`ds_rs::bigint`](./src/bigint/mod.rs)

---

- **Tests**
    - [`linked_list`](./tests/linked_list.rs)
    - [`graph`](./tests/graph.rs)
    - [`tree`](./tests/tree.rs)
    - [`matrix`](./tests/matrix.rs)
    - [`queue`](./tests/queue.rs)
    - [`stack`](./tests/stack.rs)
    - [`bigint`](./tests/bigint.rs)