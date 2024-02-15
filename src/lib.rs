#![allow(dead_code)]

use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

pub struct Queue<T> {
    vec: Vec<Option<T>>,
    size: usize,
    capacity: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Queue {
            vec: vec![None; capacity],
            size: 0,
            capacity,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn enq(&mut self, val: T) -> Result<(), ()> {
        if !self.is_full() {
            self.vec.insert(0, Some(val));
            self.size += 1;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn deq(&mut self) -> Result<(), ()> {
        if !self.is_empty() {
            self.vec[self.size - 1] = None;
            self.size -= 1;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn deq_all(&mut self) {
        self.vec = vec![None; self.capacity];
        self.size = 0;
    }
    pub fn tail(&self) -> &Option<T> {
        if !self.is_empty() {
            &self.vec[self.size - 1]
        } else {
            &None
        }
    }
    pub fn head(&self) -> &Option<T> {
        &self.vec[0]
    }
}

impl<T: Debug> Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.vec.iter().enumerate() {
            if let Some(val) = val {
                let w = writeln!(f, "{}: {:?}", idx, val);
                if let Err(e) = w {
                    return Err(e);
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn push(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node { val, next: None }));
        if self.is_empty() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            if let Some(ref tail_node) = self.tail {
                tail_node.deref().borrow_mut().next = Some(new_node.clone());
            }
            self.tail = Some(new_node);
        }
        self.size += 1;
    }
    pub fn head(&self) -> Option<T> {
        if let Some(head_node) = &self.head {
            let b = head_node.borrow();
            Some(b.val.clone())
        } else {
            None
        }
    }
    pub fn tail(&self) -> Option<T> {
        if let Some(tail_node) = &self.tail {
            let b = tail_node.borrow();
            Some(b.val.clone())
        } else {
            None
        }
    }
}
