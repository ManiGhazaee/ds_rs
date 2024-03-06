#![allow(dead_code, unused)]

use std::{collections::HashMap, hash::Hash};

pub struct Graph<K, T, W> {
    table: HashMap<K, Node<K, T, W>>,
}

pub struct Node<K, T, W> {
    key: K,
    val: Option<T>,
    neibs: HashMap<K, W>,
}

impl<K: Hash + Eq + Clone, T, W> Graph<K, T, W> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    pub fn get(&self, node_key: K) -> Option<&Node<K, T, W>> {
        self.table.get(&node_key)
    }
    pub fn get_mut(&mut self, node_key: K) -> Option<&mut Node<K, T, W>> {
        self.table.get_mut(&node_key)
    }
    pub fn contains(&self, node_key: K) -> bool {
        self.table.contains_key(&node_key)
    }
    pub fn add(&mut self, node: Node<K, T, W>) {
        self.table.insert(node.key.clone(), node);
    }
}

impl<K: Hash + Eq + Clone, T, W> Node<K, T, W> {
    pub fn new<const N: usize>(key: K, val: Option<T>, neibs: [(K, W); N]) -> Self {
        Self {
            key,
            val,
            neibs: HashMap::from(neibs),
        }
    }
    pub fn val(&self) -> Option<&T> {
        self.val.as_ref()
    }
    pub fn neibs(&self) -> &HashMap<K, W> {
        &self.neibs
    }
}
