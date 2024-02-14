#![allow(dead_code)]

struct Q<T> {
    vec: Vec<Option<T>>,
    size: usize,
    capacity: usize,
}

impl<T: Clone> Q<T> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Q {
            vec: vec![None; capacity],
            size: 0,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enq(&mut self, val: T) -> Result<(), ()> {
        if !self.is_full() {
            self.vec.insert(0, Some(val));
            self.size += 1;
            Ok(())
        } else {
            Err(())
        }
    }
    fn deq(&mut self) -> Result<(), ()> {
        if !self.is_empty() {
            self.vec[self.size] = None;
            self.size -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    fn tail(&self) -> &Option<T> {
        &self.vec[self.size]
    }
    fn head(&self) -> &Option<T> {
        &self.vec[0]
    }
}
fn main() {}
