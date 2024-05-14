use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    ops::Deref,
    rc::Rc,
};

#[derive(Debug)]
pub struct BinaryTree<T> {
    size: Rc<Cell<usize>>,
    vec: Rc<RefCell<Vec<Option<Rc<T>>>>>,
}

impl<T> BinaryTree<T> {
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b: BinaryTree<usize> = BinaryTree::new();
    /// assert!(b.is_empty());
    /// assert_eq!(b.len(), 0);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            size: Rc::new(0.into()),
            vec: Rc::new(RefCell::new(vec![])),
        }
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b: BinaryTree<usize> = BinaryTree::with_capacity(100);
    /// assert!(b.capacity() >= 100);
    /// assert_eq!(b.len(), 0);
    /// assert!(b.is_empty());
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            size: Rc::new(0.into()),
            vec: Rc::new(RefCell::new(Vec::with_capacity(capacity))),
        }
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b: BinaryTree<usize> = BinaryTree::new();
    /// assert!(b.is_empty());
    /// b.set_root(0);
    /// assert!(!b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size.get() == 0
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b: BinaryTree<usize> = BinaryTree::new();
    /// assert!(b.is_empty());
    /// b.set_root(0).set_left(2).set_right(3);
    /// assert!(!b.is_empty());
    /// assert_eq!(b.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.size.get()
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b: BinaryTree<usize> = BinaryTree::with_capacity(100);
    /// assert!(b.capacity() >= 100);
    /// assert_eq!(b.len(), 0);
    /// assert!(b.is_empty());
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.borrow().capacity()
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// b.push(1);
    /// b.push(2);
    /// b.push(3);
    /// b.push(4);
    /// assert_eq!(b.len(), 5);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// assert_eq!(b.root().left().val_clone(), Some(1));
    /// assert_eq!(b.root().right().val_clone(), Some(2));
    /// assert_eq!(b.root().left().left().val_clone(), Some(3));
    /// assert_eq!(b.root().left().right().val_clone(), Some(4));
    /// ```
    pub fn push(&mut self, val: T) {
        self.vec.borrow_mut().push(Some(Rc::new(val)));
        self.size_inc();
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// b.push(1);
    /// b.push(2);
    /// assert_eq!(b.len(), 3);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// assert_eq!(b.root().left().val_clone(), Some(1));
    /// assert_eq!(b.root().right().val_clone(), Some(2));
    /// b.pop();
    /// assert_eq!(b.len(), 2);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// assert_eq!(b.root().left().val_clone(), Some(1));
    /// assert_eq!(b.root().right().val_clone(), None);
    /// ```
    pub fn pop(&mut self) {
        self.vec.borrow_mut().pop();
        self.size_dec();
    }

    #[inline]
    fn size_inc(&self) {
        self.size.set(self.size.get() + 1);
    }

    #[inline]
    fn size_dec(&self) {
        let s = self.size.get();
        if s != 0 {
            self.size.set(s - 1);
        }
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.set_root(0);
    /// assert_eq!(b.len(), 1);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// b.pop();
    /// assert_eq!(b.root().val_clone(), None);
    /// b.push(1);
    /// assert_eq!(b.root().val_clone(), Some(1));
    /// b.set_root(0);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// ```
    #[inline]
    pub fn root(&self) -> Node<T> {
        Node::new(&self.vec, &self.size, 0)
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.set_root(0);
    /// assert_eq!(b.len(), 1);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// b.pop();
    /// assert_eq!(b.root().val_clone(), None);
    /// b.push(1);
    /// assert_eq!(b.root().val_clone(), Some(1));
    /// b.set_root(0);
    /// assert_eq!(b.root().val_clone(), Some(0));
    /// ```
    pub fn set_root(&self, val: T) -> Node<T> {
        if self.is_empty() {
            self.vec.borrow_mut().push(Some(Rc::new(val)));
            self.size_inc();
        } else {
            self.vec.borrow_mut()[0] = Some(Rc::new(val));
        }
        self.root()
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(1);
    /// b.push(2);
    /// b.push(3);
    /// assert!(!b.is_empty());
    /// b.clear();
    /// assert!(b.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.vec.borrow_mut().clear();
        self.size = Rc::new(0.into());
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(1);
    /// b.push(2);
    /// b.push(3);
    /// let v = b.as_vec();
    /// assert_eq!(v[0].val_clone(), Some(1));
    /// assert_eq!(v[1].val_clone(), Some(2));
    /// assert_eq!(v[2].val_clone(), Some(3));
    /// ```
    pub fn as_vec(&self) -> Vec<Node<T>> {
        let len = self.vec.borrow().len();
        let mut res = Vec::with_capacity(len);
        for idx in 0..self.vec.borrow().len() {
            let node = Node::new(&self.vec, &self.size, idx);
            res.push(node);
        }
        res
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// use std::rc::Rc;
    /// let mut b = BinaryTree::new();
    /// b.push(1);
    /// b.push(2);
    /// b.push(3);
    /// let v = b.as_vec_raw();
    /// assert_eq!(v[0], Some(Rc::new(1)));
    /// assert_eq!(v[1], Some(Rc::new(2)));
    /// assert_eq!(v[2], Some(Rc::new(3)));
    /// ```
    #[inline]
    pub fn as_vec_raw(&self) -> Vec<Option<Rc<T>>> {
        self.vec.borrow().clone()
    }
}

impl<T: PartialOrd> BinaryTree<T> {
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

impl<T: Clone + PartialOrd> BinaryTree<T> {
    pub fn heapify_by<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let mut input: Vec<T> = self
            .vec
            .take()
            .into_iter()
            .filter_map(|i| {
                if let Some(i) = i {
                    Some(i.deref().to_owned())
                } else {
                    None
                }
            })
            .collect();
        let input_len = input.len();
        if input_len <= 1 {
            return;
        }
        for i in (0..input.len()).rev() {
            Self::_heapify_by(&mut input, &compare, i, input_len);
        }
        *self.vec.borrow_mut() = input.into_iter().map(|i| Some(Rc::new(i))).collect();
    }

    #[inline]
    pub fn heapify_min(&mut self) {
        self.heapify_by(|a, b| b.partial_cmp(a).unwrap());
    }

    #[inline]
    pub fn heapify_max(&mut self) {
        self.heapify_by(|a, b| a.partial_cmp(b).unwrap());
    }

    #[inline]
    pub fn into_sorted_vec(self) -> Vec<T> {
        Self::into_sorted_vec_by(self, |a, b| a.partial_cmp(b).unwrap())
    }

    pub fn into_sorted_vec_by<F>(self, compare: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let mut input: Vec<T> = self
            .vec
            .take()
            .into_iter()
            .filter_map(|i| {
                if let Some(i) = i {
                    Some(i.deref().to_owned())
                } else {
                    None
                }
            })
            .collect();
        Self::heapsort_by(&mut input, compare);
        input
    }

    #[inline]
    fn heapsort_by<F>(input: &mut [T], compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        Self::_heapsort_by(input, &compare);
    }

    fn _heapsort_by<F>(input: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let input_len = input.len();
        if input_len <= 1 {
            return;
        }
        for i in (0..input.len()).rev() {
            Self::_heapify_by(input, compare, i, input_len);
        }
        for i in (1..input.len()).rev() {
            input.swap(i, 0);
            Self::_heapify_by(input, compare, 0, i);
        }
    }

    fn _heapify_by<F>(input: &mut [T], compare: &F, root: usize, len: usize)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let mut largest = root;
        let left = root * 2 + 1;
        let right = root * 2 + 2;
        if left < len {
            if let Ordering::Greater = compare(&input[left], &input[largest]) {
                largest = left;
            }
        }
        if right < len {
            if let Ordering::Greater = compare(&input[right], &input[largest]) {
                largest = right;
            }
        }
        if largest != root {
            input.swap(largest, root);
            Self::_heapify_by(input, compare, largest, len);
        }
    }
}

impl<T: Clone> BinaryTree<T> {
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(1);
    /// b.push(2);
    /// b.push(3);
    /// let v = b.into_vec();
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[1], 2);
    /// assert_eq!(v[2], 3);
    /// ```
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.vec
            .take()
            .into_iter()
            .filter_map(|i| {
                if let Some(i) = i {
                    Some(i.deref().to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for BinaryTree<T> {
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b = BinaryTree::from([1, 2, 3]);
    /// assert_eq!(b.root().val_clone(), Some(1));
    /// assert_eq!(b.root().left().val_clone(), Some(2));
    /// assert_eq!(b.root().right().val_clone(), Some(3));
    /// ```
    fn from(value: [T; N]) -> Self {
        let v: Vec<Option<Rc<T>>> = value
            .to_vec()
            .into_iter()
            .map(|i| Some(Rc::new(i)))
            .collect();
        let x = Rc::new(RefCell::new(v));
        let len = Rc::new(Cell::new(x.borrow().len()));
        BinaryTree { size: len, vec: x }
    }
}

impl<T: Clone> From<&[T]> for BinaryTree<T> {
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let v = Vec::from([1, 2, 3]);
    /// let b = BinaryTree::from(&v[..]);
    /// assert_eq!(b.root().val_clone(), Some(1));
    /// assert_eq!(b.root().left().val_clone(), Some(2));
    /// assert_eq!(b.root().right().val_clone(), Some(3));
    /// ```
    fn from(value: &[T]) -> Self {
        let v: Vec<Option<Rc<T>>> = value
            .to_vec()
            .into_iter()
            .map(|i| Some(Rc::new(i)))
            .collect();
        let x = Rc::new(RefCell::new(v));
        let len = Rc::new(Cell::new(x.borrow().len()));
        BinaryTree { size: len, vec: x }
    }
}

impl<T> From<Vec<T>> for BinaryTree<T> {
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b = BinaryTree::from(vec![1, 2, 3]);
    /// assert_eq!(b.root().val_clone(), Some(1));
    /// assert_eq!(b.root().left().val_clone(), Some(2));
    /// assert_eq!(b.root().right().val_clone(), Some(3));
    /// ```
    fn from(value: Vec<T>) -> Self {
        let v: Vec<Option<Rc<T>>> = value.into_iter().map(|i| Some(Rc::new(i))).collect();
        let x = Rc::new(RefCell::new(v));
        let len = Rc::new(Cell::new(x.borrow().len()));
        BinaryTree { size: len, vec: x }
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
    #[inline]
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

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b = BinaryTree::new();
    /// b.set_root(0);
    /// assert_eq!(b.root().left().val_clone(), None);
    /// b.root().set_left(1);
    /// assert_eq!(b.root().left().val_clone(), Some(1));
    /// ```
    pub fn left(&self) -> Self {
        let index = self.index * 2 + 1;
        Node::new(&self.vec, &self.size, index)
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let b = BinaryTree::new();
    /// b.set_root(0);
    /// assert_eq!(b.root().right().val_clone(), None);
    /// b.root().set_right(1);
    /// assert_eq!(b.root().right().val_clone(), Some(1));
    /// ```
    pub fn right(&self) -> Self {
        let index = self.index * 2 + 2;
        Node::new(&self.vec, &self.size, index)
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// b.push(1);
    /// b.push(2);
    /// b.push(3);
    /// assert_eq!(b.root().left().val_clone(), Some(1));
    /// assert_eq!(b.root().left().parent().val_clone(), Some(0));
    /// assert_eq!(b.root().left().left().val_clone(), Some(3));
    /// assert_eq!(b.root().left().left().parent().val_clone(), Some(1));
    /// ```
    pub fn parent(&self) -> Self {
        if self.is_root() {
            panic!("Node is root");
        }
        let index = (self.index - 1) / 2;
        Node::new(&self.vec, &self.size, index)
    }

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// use std::rc::Rc;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// let root: Option<Rc<isize>> = b.root().val();
    /// assert_eq!(root, Some(Rc::new(0)));
    /// ```
    pub fn val(&self) -> Option<Rc<T>> {
        if let Some(i) = self.vec.borrow().get(self.index) {
            if let Some(i) = i {
                return Some(Rc::clone(i));
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

    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// b.push(1);
    /// let root = b.root();
    /// assert!(root.is_root());
    /// assert!(root.left().parent().is_root());
    /// ```
    #[inline]
    pub const fn is_root(&self) -> bool {
        self.index == 0
    }
}

impl<T: Clone> Node<T> {
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// let root: Option<isize> = b.root().val_clone();
    /// assert_eq!(root, Some(0));
    /// ```
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
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// let r = b.root();
    /// r.set_left(1);
    /// assert_eq!(r.val_clone(), Some(0));
    /// assert_eq!(r.left().val_clone(), Some(1));
    /// r.set_left(3).set_left(4);
    /// assert_eq!(r.left().val_clone(), Some(3));
    /// assert_eq!(r.left().left().val_clone(), Some(4));
    /// ```
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
    /// # Example
    /// ```
    /// use ds_rs::tree::cell::BinaryTree;
    /// let mut b = BinaryTree::new();
    /// b.push(0);
    /// let r = b.root();
    /// r.set_right(1);
    /// assert_eq!(r.val_clone(), Some(0));
    /// assert_eq!(r.right().val_clone(), Some(1));
    /// r.set_right(3).set_right(4);
    /// assert_eq!(r.right().val_clone(), Some(3));
    /// assert_eq!(r.right().right().val_clone(), Some(4));
    /// ```
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
