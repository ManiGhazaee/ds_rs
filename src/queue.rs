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
}
