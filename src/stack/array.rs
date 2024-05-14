use std::{array, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T, const L: usize> {
    pub arr: [T; L],
    pub top: usize,
}

impl<T: Default, const L: usize> Stack<T, L> {
    pub fn new() -> Self {
        Self {
            arr: array::from_fn(|_| T::default()),
            top: L,
        }
    }

    /// # Panics
    /// if array is full
    pub fn push(&mut self, val: T) {
        self.top -= 1;
        self.arr[self.top] = val;
    }

    /// # Panics
    /// if array is empty
    pub fn pop(&mut self) -> T {
        let ret = mem::take(&mut self.arr[self.top]);
        self.top += 1;
        ret
    }

    /// # Panics
    /// if array is empty
    pub const fn peek(&self) -> &T {
        &self.arr[self.top]
    }

    /// # Panics
    /// if array is empty
    pub fn peek_mut(&mut self) -> &mut T {
        &mut self.arr[self.top]
    }

    pub fn is_full(&self) -> bool {
        self.top == 0
    }

    pub fn is_empty(&self) -> bool {
        self.top == L
    }

    pub fn iter(&self) -> std::iter::Rev<std::slice::Iter<T>> {
        self.arr[self.top..L].iter().rev()
    }

    pub fn iter_mut(&mut self) -> std::iter::Rev<std::slice::IterMut<T>> {
        self.arr[self.top..L].iter_mut().rev()
    }

    pub fn into_iter(&mut self) -> std::iter::Rev<std::slice::Iter<T>> {
        self.arr[self.top..L].into_iter().rev()
    }
}
