use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    top: Option<*mut Node<T>>,
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

impl<T> Stack<T> {
    pub const fn new() -> Self {
        Self { top: None, size: 0 }
    }

    pub fn push(&mut self, val: T) {
        let node = Node::new(val, self.top);
        self.top = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            let temp = self.top.take();
            self.top = (*temp.unwrap()).next;
            let temp = Box::from_raw(&mut (*temp.unwrap()));
            let ret = temp.val;
            self.size -= 1;
            Some(ret)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.map(|i| unsafe { &(*i).val })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.map(|i| unsafe { &mut (*i).val })
    }

    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        let mut temp = self.top;
        while let Some(n) = temp {
            unsafe {
                let b = Box::from_raw(n);
                temp = b.next;
                drop(b);
            }
        }
        self.top.take();
        self.size = 0;
    }

    #[inline]
    pub const fn iter(&self) -> Iter<T> {
        Iter {
            top: self.top,
            size: self.size,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            top: self.top,
            size: self.size,
            marker: PhantomData,
        }
    }
}

pub struct Iter<'a, T: 'a> {
    top: Option<*mut Node<T>>,
    size: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        } else {
            self.top.map(|n| unsafe {
                let ret = &(*n).val;
                self.size -= 1;
                self.top = (*n).next;
                ret
            })
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    top: Option<*mut Node<T>>,
    size: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        } else {
            self.top.map(|n| unsafe {
                let ret = &mut (*n).val;
                self.size -= 1;
                self.top = (*n).next;
                ret
            })
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
