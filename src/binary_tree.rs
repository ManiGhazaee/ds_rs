use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug)]
pub struct BinaryTree<T> {
    size: Rc<Cell<usize>>,
    vec: Rc<RefCell<Vec<Rc<T>>>>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            size: Rc::new(0.into()),
            vec: Rc::new(RefCell::new(vec![])),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size.get() == 0
    }
    pub fn len(&self) -> usize {
        self.size.get()
    }
    pub fn push(&mut self, val: T) {
        self.vec.borrow_mut().push(Rc::new(val));
        let prev = self.size.get();
        self.size.set(prev + 1);
    }
    pub fn pop(&mut self) {
        self.vec.borrow_mut().pop();
        let prev = self.size.get();
        self.size.set(prev - 1);
    }
    pub fn root_node(&self) -> Node<T> {
        Node {
            vec: Rc::clone(&self.vec),
            size: Rc::clone(&self.size),
            index: 0,
        }
    }
}

pub struct Node<T> {
    vec: Rc<RefCell<Vec<Rc<T>>>>,
    size: Rc<Cell<usize>>,
    index: usize,
}

impl<T> Node<T> {
    pub fn left(&self) -> Node<T> {
        let index = self.index * 2 + 1;
        Node {
            vec: Rc::clone(&self.vec),
            size: Rc::clone(&self.size),
            index,
        }
    }
    pub fn right(&self) -> Node<T> {
        let index = self.index * 2 + 2;
        Node {
            vec: Rc::clone(&self.vec),
            size: Rc::clone(&self.size),
            index,
        }
    }
    pub fn val(&self) -> Option<Rc<T>> {
        match self.vec.borrow().get(self.index) {
            Some(i) => Some(Rc::clone(&i)),
            None => None,
        }
    }
    /// # Panics
    /// if `self.val()` is `None`
    pub fn change(&mut self, new_val: T) {
        let mut x = self.vec.borrow_mut();
        let x = x.get_mut(self.index).unwrap();
        *x = Rc::new(new_val);
    }
}

impl<T: Clone> Node<T> {
    pub fn val_clone(&self) -> Option<T> {
        match self.val() {
            Some(i) => Some(i.as_ref().clone()),
            None => None,
        }
    }
}

impl<T: Default> Node<T> {
    /// # Returns
    /// returns the created new left node
    pub fn set_left(&self, val: T) -> Self {
        let index = self.index * 2 + 1;
        if index >= self.vec.borrow().len() {
            self.vec.borrow_mut().resize(index + 1, Default::default());
        };
        self.size.set(self.size.get() + 1);
        let mut ret = Node {
            vec: Rc::clone(&self.vec),
            size: Rc::clone(&self.size),
            index,
        };
        ret.change(val);
        ret
    }
    /// # Returns
    /// returns the created new right node
    pub fn set_right(&self, val: T) -> Self {
        let index = self.index * 2 + 2;
        if index >= self.vec.borrow().len() {
            self.vec.borrow_mut().resize(index + 1, Default::default());
        };
        self.size.set(self.size.get() + 1);
        let mut ret = Node {
            vec: Rc::clone(&self.vec),
            size: Rc::clone(&self.size),
            index,
        };
        ret.change(val);
        ret
    }
}
