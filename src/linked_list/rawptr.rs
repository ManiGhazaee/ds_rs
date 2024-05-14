use std::marker::PhantomData;

struct Node<T> {
    val: T,
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    #[inline]
    pub fn new<'a>(val: T, prev: Option<*mut Node<T>>, next: Option<*mut Node<T>>) -> &'a mut Self {
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
            size: self.size,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
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

pub struct Iter<'a, T: 'a> {
    front: Option<*mut Node<T>>,
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

pub struct IterMut<'a, T: 'a> {
    front: Option<*mut Node<T>>,
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
