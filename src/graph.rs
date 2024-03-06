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

pub struct Edge<'a, K, W> {
    from: &'a K,
    to: &'a K,
    weight: &'a W,
}

pub enum GraphInsertEdgeErr {
    Node1None,
    Node2None,
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

    /// # Returns
    /// if table did have the node.key, old value is returned
    /// if table did not have the node.key, None is returned
    pub fn insert(&mut self, node: Node<K, T, W>) -> Option<Node<K, T, W>> {
        self.table.insert(node.key.clone(), node)
    }

    pub fn remove(&mut self, node_key: K) -> Option<Node<K, T, W>> {
        self.table.remove(&node_key)
    }

    pub fn nodes(&self) -> Vec<&Node<K, T, W>> {
        self.table.values().into_iter().collect()
    }

    pub fn edges(&self) -> Vec<Edge<K, W>> {
        let mut ret: Vec<Edge<K, W>> = Vec::new();
        for i in self.table.iter() {
            for j in i.1.neibs.iter() {
                ret.push(Edge {
                    from: i.0,
                    to: j.0,
                    weight: j.1,
                })
            }
        }
        ret
    }

    pub fn insert_edge(
        &mut self,
        node1_key: K,
        node2_key: K,
        weight: W,
    ) -> Result<(), GraphInsertEdgeErr> {
        if self.table.get(&node2_key).is_some() {
            if let Some(n1) = self.table.get_mut(&node1_key) {
                n1.neibs.insert(node2_key, weight);
                Ok(())
            } else {
                Err(GraphInsertEdgeErr::Node1None)
            }
        } else {
            Err(GraphInsertEdgeErr::Node2None)
        }
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

    pub fn val_mut(&mut self) -> Option<&mut T> {
        self.val.as_mut()
    }

    pub fn change_val(&mut self, new_val: T) {
        self.val = Some(new_val);
    }

    pub fn neighbors(&self) -> &HashMap<K, W> {
        &self.neibs
    }

    pub fn change_neighbor(&mut self, neib_key: K, neib_weight: W) {
        self.neibs.insert(neib_key, neib_weight);
    }
}
