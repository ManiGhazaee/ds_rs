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
            prev: self.clone_node(&self.back),
        }));
        if self.is_empty() {
            self.front = Some(Rc::clone(&new_node));
            self.back = Some(new_node);
        } else {
            if let Some(back_node) = &self.back {
                back_node.borrow_mut().next = Some(Rc::clone(&new_node));
            }
            self.back = Some(new_node);
        }
        self.size += 1;
    }
    pub fn push_front(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: self.clone_node(&self.front),
            prev: None,
        }));

        if self.is_empty() {
            self.front = Some(Rc::clone(&new_node));
            self.back = Some(new_node);
        } else {
            self.front.clone().unwrap().borrow_mut().prev = Some(Rc::clone(&new_node));
            self.front = Some(new_node);
        }
        self.size += 1;
    }
    pub fn pop_front(&mut self) {
        if !self.is_empty() {
            if self.len() == 1 {
                self.clear();
                return;
            }
            if let Some(front) = self.clone_node(&self.front) {
                self.front = self.clone_node(&front.borrow().next);
                if let Some(front) = self.clone_node(&self.front) {
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
            if let Some(back) = self.clone_node(&self.back) {
                self.back = self.clone_node(&back.borrow().prev);
                if let Some(back) = self.clone_node(&self.back) {
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

        let before = self.get_rc(index - 1).unwrap();
        let after = self.clone_node(&before.borrow().next);
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: self.clone_node(&after),
            prev: Some(Rc::clone(&before)),
        }));
        before.borrow_mut().next = Some(Rc::clone(&new_node));
        if let Some(after) = after {
            after.borrow_mut().prev = Some(new_node);
        }

        self.size += 1;
    }
    pub fn get(&self, index: usize) -> Option<T> {
        let rc_option = self.get_rc(index);
        if let Some(val) = rc_option {
            Some(val.borrow().val.clone())
        } else {
            None
        }
    }
    fn get_rc(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if index >= self.size {
            return None;
        }
        let mut i = 0;
        let mut temp = self.clone_node(&self.front);
        while i < index {
            if let Some(t) = temp {
                let b = t.borrow();
                temp = self.clone_node(&b.next);
            }
            i += 1;
        }
        temp
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
        if let Some(other_front) = self.clone_node(&other.front) {
            other_front.borrow_mut().prev = self.clone_node(&self.back);
        } else {
            return;
        };
        let other_front = self.clone_node(&other.front);
        if let Some(back) = self.clone_node(&self.back) {
            back.borrow_mut().next = other_front;
        } else {
            self.front = other_front;
            self.back = self.clone_node(&other.back);
        }
        self.size += other.size;
        other.size = 0;
        other.back = None;
        other.front = None;
    }
    #[inline]
    fn clone_node(&self, node: &Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        option_rc_clone(node)
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

impl<T> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self {
            front: option_rc_clone(&self.front),
            back: option_rc_clone(&self.back),
            size: self.size.clone(),
        }
    }
}

pub struct Iter<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: option_rc_clone(&self.front),
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = option_rc_clone(&self.current) {
            let ret = Some(current.borrow().val.clone());
            self.current = option_rc_clone(&current.borrow().next);
            ret
        } else {
            None
        }
    }
}

#[inline]
fn option_rc_clone<T>(option: &Option<Rc<T>>) -> Option<Rc<T>> {
    match option {
        Some(val) => Some(Rc::clone(&val)),
        None => None,
    }
}
