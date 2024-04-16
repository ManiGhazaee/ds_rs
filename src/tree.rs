use std::{fmt::Debug, marker::PhantomData, ops::Deref};

#[derive(Clone, Debug)]
pub struct Node<'a, T> {
    pub val: T,
    pub parent: Option<MutPtr<'a, Node<'a, T>>>,
    pub children: Vec<MutPtr<'a, Node<'a, T>>>,
}

#[derive(Clone, Copy, Debug)]
pub struct MutPtr<'a, T> {
    pub ptr: *mut T,
    _covariant: PhantomData<&'a ()>,
}

impl<'a, T> MutPtr<'a, T> {
    pub fn new(value: &'a mut T) -> MutPtr<'a, T> {
        MutPtr {
            ptr: value,
            _covariant: PhantomData,
        }
    }
    unsafe fn as_ref(&self) -> Option<&T> {
        self.ptr.as_ref()
    }
    unsafe fn as_mut(&self) -> Option<&mut T> {
        self.ptr.as_mut()
    }
}

impl<'a, T> Deref for MutPtr<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &(*self.ptr) }
    }
}


impl<'a, T> From<*mut T> for MutPtr<'a, T> {
    fn from(value: *mut T) -> Self {
        Self {
            ptr: value,
            _covariant: PhantomData,
        }
    }
}

impl<'a, T> From<&'a mut T> for MutPtr<'a, T> {
    fn from(value: &'a mut T) -> Self {
        Self::new(value)
    }
}

impl<'a, T> Node<'a, T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            parent: None,
            children: Vec::new(),
        }
    }
    pub fn set_parent(&mut self, new_node: impl Into<MutPtr<'a, Self>>) {
        self.parent = Some(new_node.into());
    }
    pub fn set_children(&mut self, children: Vec<MutPtr<'a, Self>>) {
        self.children = children;
    }
    pub fn with_parent(mut self, new_node: impl Into<MutPtr<'a, Self>>) -> Self {
        self.parent = Some(new_node.into());
        self
    }
    pub fn with_children(mut self, children: Vec<MutPtr<'a, Self>>) -> Self {
        self.children = children;
        self
    }
    pub unsafe fn get_child(&self, index: usize) -> Option<&Self> {
        self.children.get(index)?.as_ref()
    }
    pub unsafe fn get_child_mut(&mut self, index: usize) -> Option<&mut Self> {
        self.children.get_mut(index)?.as_mut()
    }
    pub unsafe fn push_child(&mut self, new_child: impl Into<MutPtr<'a, Self>>) {
        self.children.push(new_child.into());
    }
    pub fn pop_child(&mut self) -> Option<MutPtr<Self>> {
        self.children.pop()
    }
    pub fn get_children(&self) -> &[MutPtr<Self>] {
        &self.children
    }
    pub fn get_children_mut(&mut self) -> &mut [MutPtr<'a, Self>] {
        &mut self.children
    }
}

#[derive(Debug)]
pub struct Tree<'a, T> {
    pub root: Node<'a, T>,
}

impl<'a, T> Tree<'a, T> {
    pub fn new(root: Node<'a, T>) -> Self {
        Self { root }
    }
}
