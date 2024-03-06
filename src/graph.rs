#![allow(dead_code, unused)]

use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Graph<K, T, W> {
    table: HashMap<K, Node<K, T, W>>,
}

#[derive(Debug)]
pub struct Node<K, T, W> {
    key: K,
    val: Option<T>,
    neibs: HashMap<K, W>,
}

#[derive(Debug, PartialEq)]
pub struct Edge<'a, K, W> {
    pub from: &'a K,
    pub to: &'a K,
    pub weight: &'a W,
}

#[derive(Debug, PartialEq)]
pub enum InsertEdgeErr {
    FromNone,
    ToNone,
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
        from_node_key: K,
        to_node_key: K,
        weight: W,
    ) -> Result<(), InsertEdgeErr> {
        if self.table.get(&to_node_key).is_some() {
            if let Some(n1) = self.table.get_mut(&from_node_key) {
                n1.neibs.insert(to_node_key, weight);
                Ok(())
            } else {
                Err(InsertEdgeErr::FromNone)
            }
        } else {
            Err(InsertEdgeErr::ToNone)
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

    pub fn key(&self) -> &K {
        &self.key
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

    pub fn neighbors_as_vec(&self) -> Vec<(&K, &W)> {
        self.neibs.iter().collect()
    }

    pub fn insert_neighbor(&mut self, neib_key: K, neib_weight: W) {
        self.neibs.insert(neib_key, neib_weight);
    }
}

impl<K: PartialEq + Hash + Eq + Clone, T: PartialEq, W: PartialEq> PartialEq for Node<K, T, W> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.val == other.val && self.neighbors() == other.neighbors()
    }
}

impl<'a, K, W> Edge<'a, K, W> {
    pub fn new(from: &'a K, to: &'a K, weight: &'a W) -> Self {
        Edge { from, to, weight }
    }
}
