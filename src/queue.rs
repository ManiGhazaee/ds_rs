use std::fmt::Debug;

pub struct Queue<T> {
    vec: Vec<Option<T>>,
    size: usize,
    capacity: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Queue {
            vec: vec![None; capacity],
            size: 0,
            capacity,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn enq(&mut self, val: T) -> Result<(), ()> {
        if !self.is_full() {
            self.vec.insert(0, Some(val));
            self.size += 1;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn deq(&mut self) -> Result<(), ()> {
        if !self.is_empty() {
            self.vec[self.size - 1] = None;
            self.size -= 1;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn deq_all(&mut self) {
        self.vec = vec![None; self.capacity];
        self.size = 0;
    }
    pub fn tail(&self) -> Option<&T> {
        if !self.is_empty() {
            self.vec[self.size - 1].as_ref()
        } else {
            None
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.vec[0].as_ref()
    }
}

impl<T: Debug> Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.vec.iter().enumerate() {
            if let Some(val) = val {
                let w = writeln!(f, "{}: {:?}", idx, val);
                if let Err(e) = w {
                    return Err(e);
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
