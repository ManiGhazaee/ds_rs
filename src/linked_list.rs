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
    #[inline]
    pub const fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            size: 0,
        }
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }
    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: self.clone_node(self.back.as_ref()),
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
            next: self.clone_node(self.front.as_ref()),
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
    pub fn pop_front(&mut self) -> Option<T> {
        if !self.is_empty() {
            if self.len() == 1 {
                let ret = self.front();
                self.clear();
                return ret;
            }
            if let Some(front) = self.clone_node(self.front.as_ref()) {
                let ret = self.clone_node(self.front.as_ref());
                self.front = self.clone_node(front.borrow().next.as_ref());
                if let Some(front) = self.clone_node(self.front.as_ref()) {
                    front.borrow_mut().prev = None;
                }
                self.size -= 1;
                return self.node_val(ret.as_ref());
            }
        }
        None
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if !self.is_empty() {
            if self.len() == 1 {
                let ret = self.back();
                self.clear();
                return ret;
            }
            if let Some(back) = self.clone_node(self.back.as_ref()) {
                let ret = self.clone_node(self.back.as_ref());
                self.back = self.clone_node(back.borrow().prev.as_ref());
                if let Some(back) = self.clone_node(self.back.as_ref()) {
                    back.borrow_mut().next = None;
                }
                self.size -= 1;
                return self.node_val(ret.as_ref());
            }
        }
        None
    }
    pub fn clear(&mut self) {
        let mut current_node = self.front.take();
        while let Some(node) = current_node {
            if let Some(next) = node.borrow_mut().next.take() {
                current_node = Some(next);
            } else {
                break;
            }
        }
        self.back = None;
        self.size = 0;
    }
    /// # Panics
    /// if `index > len`
    pub fn insert(&mut self, index: usize, val: T) {
        if index > self.size {
            panic!("index > len");
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
        let after = self.clone_node(before.borrow().next.as_ref());
        let new_node = Rc::new(RefCell::new(Node {
            val,
            next: self.clone_node(after.as_ref()),
            prev: Some(Rc::clone(&before)),
        }));
        before.borrow_mut().next = Some(Rc::clone(&new_node));
        if let Some(after) = after {
            after.borrow_mut().prev = Some(new_node);
        }

        self.size += 1;
    }
    pub fn get(&self, index: usize) -> Option<T> {
        self.node_val(self.get_rc(index).as_ref())
    }
    fn get_rc(&self, index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if index >= self.size {
            return None;
        }
        let mut i = 0;
        let mut temp = self.clone_node(self.front.as_ref());
        while i < index {
            if let Some(t) = temp {
                let b = t.borrow();
                temp = self.clone_node(b.next.as_ref());
            }
            i += 1;
        }
        temp
    }
    #[inline]
    pub fn front(&self) -> Option<T> {
        self.node_val(self.front.as_ref())
    }
    #[inline]
    pub fn back(&self) -> Option<T> {
        self.node_val(self.back.as_ref())
    }
    pub fn append(&mut self, other: &mut LinkedList<T>) {
        if let Some(other_front) = self.clone_node(other.front.as_ref()) {
            other_front.borrow_mut().prev = self.clone_node(self.back.as_ref());
        } else {
            return;
        };
        let other_front = self.clone_node(other.front.as_ref());
        if let Some(back) = self.clone_node(self.back.as_ref()) {
            back.borrow_mut().next = other_front;
        } else {
            self.front = other_front;
            self.back = self.clone_node(other.back.as_ref());
        }
        self.size += other.size;
        other.size = 0;
        other.back = None;
        other.front = None;
    }
    /// # Panics
    /// if `index >= len`
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            panic!("index > len");
        }
        if index == 0 {
            return self.pop_front();
        }
        if self.size <= 1 || index == self.size - 1 {
            return self.pop_back();
        }
        let before = self.get_rc(index - 1).unwrap();
        let current = self.clone_node(before.borrow().next.as_ref());
        let after = self.clone_node(
            self.clone_node(current.as_ref())
                .unwrap()
                .borrow()
                .next
                .as_ref(),
        );
        before.borrow_mut().next = self.clone_node(after.as_ref());
        after.unwrap().borrow_mut().prev = self.clone_node(Some(&before));

        self.size -= 1;
        self.node_val(current.as_ref())
    }
    /// # Panics
    /// if `index >= len`
    pub fn change(&mut self, index: usize, new_val: T) {
        if index >= self.size {
            panic!("index > len");
        }
        let node = self.get_rc(index).unwrap();
        node.borrow_mut().val = new_val;
    }
    #[inline]
    fn clone_node(&self, node: Option<&Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        option_rc_clone(node)
    }
    #[inline]
    fn node_val(&self, node: Option<&Rc<RefCell<Node<T>>>>) -> Option<T> {
        if let Some(node) = node {
            Some(node.borrow().val.clone())
        } else {
            None
        }
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
        let mut temp = self.clone_node(self.front.as_ref());
        while let Some(n) = temp {
            let b = n.borrow();
            temp = self.clone_node(b.next.as_ref());
            let val = &b.val;
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
            front: option_rc_clone(self.front.as_ref()),
            back: option_rc_clone(self.back.as_ref()),
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
            current: option_rc_clone(self.front.as_ref()),
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = option_rc_clone(self.current.as_ref()) {
            let b = current.borrow();
            let ret = Some(b.val.clone());
            self.current = option_rc_clone(b.next.as_ref());
            ret
        } else {
            None
        }
    }
}

#[inline]
fn option_rc_clone<T>(option: Option<&Rc<T>>) -> Option<Rc<T>> {
    match option {
        Some(val) => Some(Rc::clone(val)),
        None => None,
    }
}

pub mod rawptr {
    use std::marker::PhantomData;

    struct Node<T> {
        val: T,
        next: Option<*mut Node<T>>,
        prev: Option<*mut Node<T>>,
    }

    impl<T> Node<T> {
        #[inline]
        pub fn new<'a>(
            val: T,
            prev: Option<*mut Node<T>>,
            next: Option<*mut Node<T>>,
        ) -> &'a mut Self {
            let node = Node { val, next, prev };
            let b = Box::new(node);
            Box::leak(b)
        }
    }

    impl<T: Default> Default for Node<T> {
        fn default() -> Self {
            Self {
                val: Default::default(),
                next: Default::default(),
                prev: Default::default(),
            }
        }
    }

    #[derive(Debug)]
    pub struct LinkedList<T> {
        front: Option<*mut Node<T>>,
        back: Option<*mut Node<T>>,
        size: usize,
    }

    impl<T> LinkedList<T> {
        #[inline]
        pub const fn new() -> Self {
            Self {
                front: None,
                back: None,
                size: 0,
            }
        }

        #[inline]
        pub const fn is_empty(&self) -> bool {
            self.size == 0
        }

        #[inline]
        pub fn len(&self) -> usize {
            self.size
        }

        pub fn front(&self) -> Option<&T> {
            if let Some(f) = self.front {
                unsafe { Some(&(*f).val) }
            } else {
                None
            }
        }
        pub fn back(&self) -> Option<&T> {
            if let Some(b) = self.back {
                unsafe { Some(&(*b).val) }
            } else {
                None
            }
        }

        pub fn front_mut(&mut self) -> Option<&mut T> {
            if let Some(f) = self.front {
                unsafe { Some(&mut (*f).val) }
            } else {
                None
            }
        }
        pub fn back_mut(&mut self) -> Option<&mut T> {
            if let Some(b) = self.back {
                unsafe { Some(&mut (*b).val) }
            } else {
                None
            }
        }

        pub fn push_back(&mut self, val: T) {
            let node = Node::new(val, self.back, None);
            if self.is_empty() {
                self.back = Some(node);
                self.front = Some(node);
            } else {
                unsafe {
                    (*self.back.unwrap()).next = Some(node);
                    self.back = Some(node);
                }
            }
            self.size += 1;
        }

        pub fn push_front(&mut self, val: T) {
            let node = Node::new(val, None, self.front);
            if self.is_empty() {
                self.back = Some(node);
                self.front = Some(node);
            } else {
                unsafe {
                    (*self.front.unwrap()).prev = Some(node);
                    self.front = Some(node);
                }
            }
            self.size += 1;
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }
            unsafe {
                let temp = self.back.take();
                self.back = (*temp.unwrap()).prev;
                if let Some(back) = self.back {
                    (*back).next = None;
                } else {
                    self.front = None;
                }
                let temp = Box::from_raw(&mut (*temp.unwrap()));
                let ret = temp.val;
                self.size -= 1;
                Some(ret)
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }
            unsafe {
                let temp = self.front.take();
                self.front = (*temp.unwrap()).next;
                if let Some(front) = self.front {
                    (*front).prev = None;
                } else {
                    self.back = None;
                }
                let temp = Box::from_raw(&mut (*temp.unwrap()));
                let ret = temp.val;
                self.size -= 1;
                Some(ret)
            }
        }

        #[inline]
        pub fn get(&self, index: usize) -> Option<&T> {
            self.iter().nth(index)
        }

        #[inline]
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            self.iter_mut().nth(index)
        }

        fn get_node_mut(&mut self, index: usize) -> Option<*mut Node<T>> {
            let mut temp = self.front;
            let mut i = 0;
            while i < index {
                if let Some(n) = temp {
                    unsafe {
                        temp = (*n).next;
                    }
                } else {
                    return None;
                }
                i += 1;
            }
            temp
        }

        /// # Panics
        /// if `index > len`
        pub fn insert(&mut self, index: usize, val: T) {
            if index > self.size {
                panic!("index > len");
            }
            if index == 0 {
                self.push_front(val);
                return;
            }
            if index == self.size {
                self.push_back(val);
                return;
            }
            if let Some(n) = self.get_node_mut(index) {
                unsafe {
                    let node = Node::new(val, (*n).prev, Some(n));
                    if let Some(prev) = (*n).prev {
                        (*prev).next = Some(node);
                    }
                    (*n).prev = Some(node);
                }
                self.size += 1;
            }
        }

        pub fn remove(&mut self, index: usize) -> Option<T> {
            if index >= self.size {
                panic!("index > len");
            }
            if index == 0 {
                return self.pop_front();
            }
            if self.size <= 1 || index == self.size - 1 {
                return self.pop_back();
            }
            self.get_node_mut(index).map(|n| unsafe {
                if let Some(prev) = (*n).prev {
                    (*prev).next = (*n).next;
                }
                if let Some(next) = (*n).next {
                    (*next).prev = (*n).prev;
                }
                let b = Box::from_raw(n);
                self.size -= 1;
                b.val
            })
        }

        pub fn append(&mut self, other: &mut Self) {
            unsafe {
                if let Some(other_front) = other.front {
                    (*other_front).prev = self.back;
                }
                if let Some(back) = self.back {
                    (*back).next = other.front;
                } else {
                    self.front = other.front;
                    self.back = other.back;
                }
                self.size += other.size;
                other.size = 0;
                other.front = None;
                other.back = None;
            }
        }

        #[inline]
        pub const fn iter(&self) -> Iter<T> {
            Iter {
                front: self.front,
                back: self.back,
                size: self.size,
                marker: PhantomData,
            }
        }

        #[inline]
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut {
                front: self.front,
                back: self.back,
                size: self.size,
                marker: PhantomData,
            }
        }

        pub fn clear(&mut self) {
            let mut temp = self.front;
            while let Some(n) = temp {
                unsafe {
                    let b = Box::from_raw(n);
                    temp = b.next;
                    drop(b);
                }
            }
            self.front.take();
            self.back.take();
            self.size = 0;
        }
    }

    impl<T: PartialEq> LinkedList<T> {
        pub fn contains(&self, val: &T) -> bool {
            self.iter().any(|i| i == val)
        }
    }

    #[allow(dead_code)]
    pub struct Iter<'a, T: 'a> {
        front: Option<*mut Node<T>>,
        back: Option<*mut Node<T>>,
        size: usize,
        marker: PhantomData<&'a Node<T>>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.size == 0 {
                return None;
            } else {
                self.front.map(|n| unsafe {
                    let ret = &(*n).val;
                    self.size -= 1;
                    self.front = (*n).next;
                    ret
                })
            }
        }
    }

    #[allow(dead_code)]
    pub struct IterMut<'a, T: 'a> {
        front: Option<*mut Node<T>>,
        back: Option<*mut Node<T>>,
        size: usize,
        marker: PhantomData<&'a mut Node<T>>,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.size == 0 {
                return None;
            } else {
                self.front.map(|n| unsafe {
                    let ret = &mut (*n).val;
                    self.size -= 1;
                    self.front = (*n).next;
                    ret
                })
            }
        }
    }

    impl<T> Drop for LinkedList<T> {
        fn drop(&mut self) {
            self.clear();
        }
    }
}
