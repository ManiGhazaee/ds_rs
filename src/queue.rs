use std::array;

#[derive(Debug)]
pub struct Queue<T, const L: usize> {
    arr: [T; L],
    back: usize,
    front: usize,
    size: usize,
}

impl<T: Clone + Default, const L: usize> Queue<T, L> {
    pub fn new() -> Self {
        Queue {
            arr: array::from_fn(|_| T::default()),
            back: L,
            front: L,
            size: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn is_full(&self) -> bool {
        self.size == L
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn capacity(&self) -> usize {
        L
    }
    /// # Panics
    /// if array is full
    pub fn enq(&mut self, val: T) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.size += 1;
        if self.size == 1 {
            self.back -= 1;
            self.front -= 1;
        } else if self.back == 0 {
            self.back = L - 1;
        } else {
            self.back -= 1;
        }
        self.arr[self.back] = val;
    }
    /// # Panics
    /// if array is empty
    pub fn deq(&mut self) -> T {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        let ret = self.arr[self.front].clone();
        self.size -= 1;
        if self.size == 0 {
            self.front = L;
            self.back = L;
        } else {
            if self.front == 0 {
                self.front = L - 1;
            } else {
                self.front -= 1;
            }
        }
        ret
    }
    pub fn clear(&mut self) {
        self.size = 0;
        self.back = L;
        self.front = L;
    }
    pub fn front(&self) -> Option<&T> {
        self.arr.get(self.front)
    }
    pub fn back(&self) -> Option<&T> {
        self.arr.get(self.back)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, T, L> {
        Iter {
            arr: &self.arr,
            index: self.front,
            size: self.size,
        }
    }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T, L> {
        IterMut {
            arr: &mut self.arr,
            index: self.front,
            size: self.size,
        }
    }
}

pub struct Iter<'a, T, const L: usize> {
    arr: &'a [T; L],
    index: usize,
    size: usize,
}

impl<'a, T, const L: usize> Iterator for Iter<'a, T, L> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        let ret = &self.arr[self.index];
        if self.index == 0 {
            self.index = L - 1;
        } else {
            self.index -= 1;
        };
        self.size -= 1;

        Some(ret)
    }
}

pub struct IterMut<'a, T, const L: usize> {
    arr: &'a mut [T; L],
    index: usize,
    size: usize,
}

impl<'a, T, const L: usize> Iterator for IterMut<'a, T, L> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        let ret = &mut self.arr[self.index] as *mut T;
        if self.index == 0 {
            self.index = L - 1;
        } else {
            self.index -= 1;
        };
        self.size -= 1;

        Some(unsafe { &mut *ret })
    }
}

impl<'a, T, const L: usize> IntoIterator for &'a Queue<T, L>
where
    T: Clone + Default,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T, L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const L: usize> IntoIterator for &'a mut Queue<T, L>
where
    T: Clone + Default,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct IntoIter<T, const L: usize> {
    arr: [T; L],
    index: usize,
    size: usize,
}

impl<T, const L: usize> Iterator for IntoIter<T, L>
where
    T: Clone + Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        let ret = self.arr[self.index].clone();
        if self.index == 0 {
            self.index = L - 1;
        } else {
            self.index -= 1;
        };
        self.size -= 1;

        Some(ret)
    }
}

impl<T, const L: usize> IntoIterator for Queue<T, L>
where
    T: Clone + Default,
{
    type Item = T;
    type IntoIter = IntoIter<T, L>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            arr: self.arr,
            index: self.front,
            size: self.size,
        }
    }
}
