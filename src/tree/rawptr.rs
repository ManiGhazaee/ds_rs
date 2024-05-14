use std::collections::VecDeque;

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<*mut Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    left: Option<*mut Node<T>>,
    right: Option<*mut Node<T>>,
    parent: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    #[inline]
    fn new<'a>(
        val: T,
        left: Option<*mut Node<T>>,
        right: Option<*mut Node<T>>,
        parent: Option<*mut Node<T>>,
    ) -> &'a mut Self {
        let node = Node {
            val,
            left,
            right,
            parent,
        };
        let b = Box::new(node);
        Box::leak(b)
    }

    pub fn left(&self) -> Option<&Self> {
        unsafe { self.left.map(|i| &(*i)) }
    }

    pub fn right(&self) -> Option<&Self> {
        unsafe { self.right.map(|i| &(*i)) }
    }

    pub fn left_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.left.map(|i| &mut (*i)) }
    }

    pub fn right_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.right.map(|i| &mut (*i)) }
    }

    pub fn set_left(&mut self, new_val: T) -> &Self {
        self.left = Some(Node::new(new_val, None, None, Some(&mut (*self))));
        unsafe { &*self.left.unwrap() }
    }

    pub fn set_right(&mut self, new_val: T) -> &Self {
        self.right = Some(Node::new(new_val, None, None, Some(&mut (*self))));
        unsafe { &*self.right.unwrap() }
    }

    pub fn set_left_mut(&mut self, new_val: T) -> &mut Self {
        self.left = Some(Node::new(new_val, None, None, Some(&mut (*self))));
        unsafe { &mut *self.left.unwrap() }
    }

    pub fn set_right_mut(&mut self, new_val: T) -> &mut Self {
        self.right = Some(Node::new(new_val, None, None, Some(&mut (*self))));
        unsafe { &mut *self.right.unwrap() }
    }

    pub const fn val(&self) -> &T {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut T {
        &mut self.val
    }

    pub fn parent(&self) -> Option<&Self> {
        unsafe { self.parent.map(|i| &(*i)) }
    }

    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.parent.map(|i| &mut (*i)) }
    }
}

impl<T> BinaryTree<T> {
    pub const fn new() -> Self {
        Self { root: None }
    }

    pub const fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn root(&self) -> Option<&Node<T>> {
        unsafe { self.root.map(|i| &(*i)) }
    }

    pub fn root_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe { self.root.map(|i| &mut (*i)) }
    }

    pub fn set_root(&mut self, new_val: T) -> &Node<T> {
        self.root = Some(Node::new(new_val, None, None, None));
        unsafe { &*self.root.unwrap() }
    }

    pub fn set_root_mut(&mut self, new_val: T) -> &mut Node<T> {
        self.root = Some(Node::new(new_val, None, None, None));
        unsafe { &mut *self.root.unwrap() }
    }

    pub fn clear(&mut self) {
        if let Some(r) = self.root {
            let mut q = VecDeque::from([r]);
            while let Some(n) = q.pop_front() {
                unsafe {
                    let b = Box::from_raw(n);
                    if let Some(n) = (*b).left {
                        q.push_back(n);
                    }
                    if let Some(n) = (*b).right {
                        q.push_back(n);
                    }
                    drop(b);
                }
            }
            self.root = None;
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
            && self.left == other.left
            && self.right == other.right
            && self.parent == other.parent
    }
}
