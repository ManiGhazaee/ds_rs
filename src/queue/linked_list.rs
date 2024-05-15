use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue<T> {
    front: Option<*mut Node<T>>,
    back: Option<*mut Node<T>>,
    size: usize,
}

struct Node<T> {
    val: T,
    next: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    #[inline]
    pub fn new<'a>(val: T, next: Option<*mut Node<T>>) -> &'a mut Self {
        let node = Node { val, next };
        let b = Box::new(node);
        Box::leak(b)
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            back: None,
            front: None,
            size: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub const fn size(&self) -> usize {
        self.size
    }

    pub fn enq(&mut self, val: T) {
        let node = Node::new(val, None);
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

    pub fn deq(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            let temp = self.front.take();
            self.front = (*temp.unwrap()).next;
            if self.front.is_none() {
                self.back = None;
            };
            let temp = Box::from_raw(&mut (*temp.unwrap()));
            let ret = temp.val;
            self.size -= 1;
            Some(ret)
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

    pub fn front(&self) -> Option<&T> {
        self.front.map(|i| unsafe { &(*i).val })
    }

    pub fn back(&self) -> Option<&T> {
        self.back.map(|i| unsafe { &(*i).val })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.front.map(|i| unsafe { &mut (*i).val })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.back.map(|i| unsafe { &mut (*i).val })
    }

    /// iterates from front to back
    pub const fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            front: self.front,
            size: self.size,
            marker: PhantomData,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            size: self.size,
            front: self.front,
            marker: PhantomData,
        }
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

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
