use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    rc::Rc,
};

#[derive(Debug)]
pub struct BinaryTree<T> {
    size: Rc<Cell<usize>>,
    vec: Rc<RefCell<Vec<Option<Rc<T>>>>>,
}

impl<T> BinaryTree<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            size: Rc::new(0.into()),
            vec: Rc::new(RefCell::new(vec![])),
        }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            size: Rc::new(0.into()),
            vec: Rc::new(RefCell::new(Vec::with_capacity(capacity))),
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size.get() == 0
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.size.get()
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.borrow().capacity()
    }
    pub fn push(&mut self, val: T) {
        self.vec.borrow_mut().push(Some(Rc::new(val)));
        self.size.set(self.size.get() + 1);
    }
    pub fn pop(&mut self) {
        self.vec.borrow_mut().pop();
        let s = self.size.get();
        if s != 0 {
            self.size.set(s - 1);
        } 
    }
    #[inline]
    pub fn root(&self) -> Node<T> {
        Node::new(&self.vec, &self.size, 0)
    }
    pub fn set_root(&self, val: T) -> Node<T> {
        if self.is_empty() {
            self.vec.borrow_mut().push(Some(Rc::new(val)));
        } else {
            self.vec.borrow_mut()[0] = Some(Rc::new(val));
        }
        self.root()
    }
    pub fn clear(&mut self) {
        self.vec.borrow_mut().clear();
        self.size = Rc::new(0.into());
    }
    pub fn as_vec(&self) -> Vec<Node<T>> {
        let len = self.vec.borrow().len();
        let mut res = Vec::with_capacity(len);
        for idx in 0..self.vec.borrow().len() {
            let node = Node::new(&self.vec, &self.size, idx);
            res.push(node);
        }
        res
    }
    pub fn as_vec_raw(&self) -> Vec<Option<Rc<T>>> {
        self.vec.borrow().clone()
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    #[inline]
    pub fn heapify_min(&mut self) {
        self.heapify_by(|a, b| b.partial_cmp(a).unwrap());
    }
    #[inline]
    pub fn heapify_max(&mut self) {
        self.heapify_by(|a, b| a.partial_cmp(b).unwrap());
    }
    pub fn heapify_by<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        if self.size.get() <= 1 {
            return;
        }
        let len = self.vec.borrow().len();
        for i in (0..len).rev() {
            self._heapify_by(&compare, i);
        }
    }
    fn _heapify_by<F>(&mut self, compare: &F, root: usize)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let root = Node::new(&self.vec, &self.size, root);
        let mut largest = root.clone();
        let left = largest.left();
        let right = largest.right();
        if let Some(_left) = left.val() {
            if let Ordering::Greater = compare(&_left, &largest.val().unwrap()) {
                largest = left;
            }
        }
        if let Some(_right) = right.val() {
            if let Ordering::Greater = compare(&_right, &largest.val().unwrap()) {
                largest = right;
            }
        }
        if largest != root {
            self.vec.borrow_mut().swap(largest.index, root.index);
            self._heapify_by(compare, largest.index);
        }
    }
    pub fn is_heap_by<F>(&self, compare: F) -> bool
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let len = self.vec.borrow().len();
        if len <= 1 {
            return true;
        }
        let mut i = len - 1;
        loop {
            if i == 0 {
                break;
            };
            let parent = &self.vec.borrow()[(i - 1) / 2];
            let current = &self.vec.borrow()[i];
            if let (Some(c), Some(p)) = (current, parent) {
                if let Ordering::Less = compare(&*c, &*p) {
                    return false;
                }
            } else {
                continue;
            }
            i -= 1;
        }
        true
    }
    #[inline]
    pub fn is_max_heap(&self) -> bool {
        self.is_heap_by(|a, b| b.partial_cmp(a).unwrap())
    }
    #[inline]
    pub fn is_min_heap(&self) -> bool {
        self.is_heap_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

pub struct Node<T> {
    vec: Rc<RefCell<Vec<Option<Rc<T>>>>>,
    size: Rc<Cell<usize>>,
    index: usize,
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec.as_ptr() == other.vec.as_ptr()
            && self.size == other.size
            && self.index == other.index
    }
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            vec: Rc::clone(&self.vec),
            size: Rc::clone(&self.size),
            index: self.index.clone(),
        }
    }
}

impl<T> Node<T> {
    #[inline]
    fn new(vec: &Rc<RefCell<Vec<Option<Rc<T>>>>>, size: &Rc<Cell<usize>>, index: usize) -> Self {
        Self {
            vec: Rc::clone(vec),
            size: Rc::clone(size),
            index,
        }
    }
    pub fn left(&self) -> Node<T> {
        let index = self.index * 2 + 1;
        Node::new(&self.vec, &self.size, index)
    }
    pub fn right(&self) -> Node<T> {
        let index = self.index * 2 + 2;
        Node::new(&self.vec, &self.size, index)
    }
    pub fn parent(&self) -> Node<T> {
        if self.is_root() {
            panic!("Node is root");
        }
        let index = (self.index - 1) / 2;
        Node::new(&self.vec, &self.size, index)
    }
    pub fn val(&self) -> Option<Rc<T>> {
        if let Some(i) = self.vec.borrow().get(self.index) {
            if let Some(i) = i {
                return Some(Rc::clone(&i));
            }
        }
        None
    }
    /// # Panics
    /// if `self.val()` is `None`
    pub fn change(&mut self, new_val: T) {
        let mut x = self.vec.borrow_mut();
        let x = x.get_mut(self.index).unwrap();
        *x = Some(Rc::new(new_val));
    }
    #[inline]
    pub const fn is_root(&self) -> bool {
        self.index == 0
    }
}

impl<T: Clone> Node<T> {
    #[inline]
    pub fn val_clone(&self) -> Option<T> {
        match self.val() {
            Some(i) => Some((*i).clone()),
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
        let mut ret = Node::new(&self.vec, &self.size, index);
        if let None = ret.val() {
            self.size.set(self.size.get() + 1);
        }
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
        let mut ret = Node::new(&self.vec, &self.size, index);
        if let None = ret.val() {
            self.size.set(self.size.get() + 1);
        }
        ret.change(val);
        ret
    }
}
