use std::array;

pub struct Stack<T, const L: usize> {
    pub arr: [T; L],
    pub top: isize,
}

impl<T: Clone + Default, const L: usize> Stack<T, L> {
    pub fn new() -> Self {
        Self {
            arr: array::from_fn(|_| T::default()),
            top: -1,
        }
    }
    /// # Panics
    /// if top + 1 == L
    pub fn push(&mut self, val: T) {
        self.top += 1;
        self.arr[self.top as usize] = val;
    }
    /// # Panics 
    /// if top < 0 
    pub fn pop(&mut self) -> T {
        let ret = self.arr[self.top as usize].clone();
        self.top -= 1;
        ret
    }
    /// # Panics
    /// if top < 0
    pub fn peak(&self) -> &T {
        &self.arr[self.top as usize]
    }
    pub fn is_full(&self) -> bool {
        self.top as usize + 1 == L
    }
    pub fn is_empty(&self) -> bool {
        self.top == -1
    }
}
