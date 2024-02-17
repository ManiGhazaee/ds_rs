use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

pub struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T> {
    front: Option<Rc<RefCell<Node<T>>>>,
    back: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            size: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: self.back.clone(),
        }));
        if self.is_empty() {
            self.front = Some(new_node.clone());
            self.back = Some(new_node);
        } else {
            if let Some(back_node) = self.back.clone() {
                back_node.borrow_mut().next = Some(new_node.clone());
            }
            self.back = Some(new_node);
        }
        self.size += 1;
    }
    pub fn push_front(&mut self, val: T) {
        let new_node = Some(Rc::new(RefCell::new(Node {
            val,
            next: self.front.clone(),
            prev: None,
        })));

        if self.is_empty() {
            self.front = new_node.clone();
            self.back = new_node;
        } else {
            self.front.clone().unwrap().borrow_mut().prev = new_node.clone();
            self.front = new_node;
        }
        self.size += 1;
    }
    pub fn pop_front(&mut self) {
        if !self.is_empty() {
            if self.len() == 1 {
                self.clear();
                return;
            }
            if let Some(front) = self.front.clone() {
                self.front = front.borrow().next.clone();
                if let Some(front) = self.front.clone() {
                    front.borrow_mut().prev = None;
                }
                self.size -= 1;
            }
        }
    }
    pub fn pop_back(&mut self) {
        if !self.is_empty() {
            if self.len() == 1 {
                self.clear();
                return;
            }
            if let Some(back) = self.back.clone() {
                self.back = back.borrow().prev.clone();
                if let Some(back) = self.back.clone() {
                    back.borrow_mut().next = None;
                }
                self.size -= 1;
            }
        }
    }
    pub fn clear(&mut self) {
        self.front = None;
        self.back = None;
        self.size = 0;
    }
    /// # Panics
    /// if `index > size`
    pub fn insert(&mut self, index: usize, val: T) {
        if index > self.size {
            panic!("index > size");
        }
        if index == 0 {
            self.push_front(val);
            return;
        }
        if index == self.size {
            self.push_back(val);
            return;
        }
        let mut i = 0;
        let mut before = self.front.clone();
        while i < index - 1 {
            if let Some(t) = before {
                let b = t.borrow();
                before = b.next.clone();
            }
            i += 1;
        }
        let ref_cell = before.clone().unwrap();
        let after = ref_cell.borrow().next.clone().unwrap();
        let before = before.unwrap();
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: Some(after),
            prev: Some(before.clone()),
        }));
        before.borrow_mut().next = Some(new_node.clone());
        self.size += 1;
    }
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        let mut i = 0;
        let mut temp = self.front.clone();
        while i < index {
            if let Some(t) = temp {
                let b = t.borrow();
                temp = b.next.clone();
            }
            i += 1;
        }
        let temp = temp.unwrap();
        let ret = Some(temp.borrow().val.clone());
        ret
    }
    pub fn front(&self) -> Option<T> {
        if let Some(front_node) = &self.front {
            let b = front_node.borrow();
            Some(b.val.clone())
        } else {
            None
        }
    }
    pub fn back(&self) -> Option<T> {
        if let Some(back_node) = &self.back {
            let b = back_node.borrow();
            Some(b.val.clone())
        } else {
            None
        }
    }
    pub fn append(&mut self, other: &mut LinkedList<T>) {
        let other_front = other.front.clone();
        other_front.clone().unwrap().borrow_mut().prev = self.back.clone();
        self.back.clone().unwrap().borrow_mut().next = other_front;
        self.size += other.size;
        other.size = 0;
        other.back = None;
        other.front = None;
    }
}

impl<T: PartialEq> LinkedList<T> {
    pub fn contains(&self, val: T) -> bool {
        let mut temp = self.front.clone();
        while let Some(n) = temp {
            if val == n.borrow().val {
                return true;
            }
            temp = n.borrow().next.clone();
        }
        false
    }
}

impl<T: Debug + Clone> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = 0;
        let mut temp = self.front.clone();
        while let Some(n) = temp {
            let b = n.borrow();
            temp = b.next.clone();
            let val = b.val.clone();
            let w = writeln!(f, "{}: {:?}", idx, val);
            if let Err(e) = w {
                return Err(e);
            }
            idx += 1;
        }
        Ok(())
    }
}
