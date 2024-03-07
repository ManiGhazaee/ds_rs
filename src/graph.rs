#![allow(dead_code, unused)]

use std::{collections::{hash_map, HashMap}, fmt::{Debug, Display, Formatter}, hash::Hash};

#[derive(Debug)]
pub struct Graph<K, T, W> {
    map: HashMap<K, Node<K, T, W>>,
}

#[derive(Debug)]
pub struct Node<K, T, W> {
    key: K,
    val: T,
    neibs: HashMap<K, W>,
}

#[derive(Debug, PartialEq)]
pub struct Edge<'a, K, W> {
    pub from: &'a K,
    pub to: &'a K,
    pub weight: &'a W,
}

#[derive(Debug, PartialEq)]
pub enum EdgeErr {
    FromNone,
    ToNone,
}

impl<K: Hash + Eq + Clone, T, W> Graph<K, T, W> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, node_key: K) -> Option<&Node<K, T, W>> {
        self.map.get(&node_key)
    }

    pub fn get_mut(&mut self, node_key: K) -> Option<&mut Node<K, T, W>> {
        self.map.get_mut(&node_key)
    }

    pub fn contains(&self, node_key: K) -> bool {
        self.map.contains_key(&node_key)
    }

    /// # Returns
    /// if map did have the node.key, old value is returned
    /// if map did not have the node.key, None is returned
    pub fn insert(&mut self, node: Node<K, T, W>) -> Option<Node<K, T, W>> {
        self.map.insert(node.key.clone(), node)
    }

    pub fn remove(&mut self, node_key: K) -> Option<Node<K, T, W>> {
        self.map.remove(&node_key)
    }

    pub fn nodes(&self) -> Vec<&Node<K, T, W>> {
        self.map.values().into_iter().collect()
    }

    pub fn nodes_len(&self) -> usize {
        self.map.keys().len()
    }

    pub fn edges(&self) -> Vec<Edge<K, W>> {
        let mut ret: Vec<Edge<K, W>> = Vec::new();
        for i in self.map.iter() {
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

    pub fn edges_len(&self) -> usize {
        let mut ret = 0;
        for i in self.map.iter() {
            ret += i.1.neibs.keys().len();
        }
        ret
    }

    /// # Error
    /// if graph doesn't contain to_node_key returns `Err(EdgeErr::ToNone)`.
    /// 
    /// if graph doesn't contain from_node_key returns `Err(EdgeErr::FromNone)`.
    pub fn insert_edge(
        &mut self,
        from_node_key: K,
        to_node_key: K,
        weight: W,
    ) -> Result<(), EdgeErr> {
        if self.map.contains_key(&to_node_key) {
            if let Some(n1) = self.map.get_mut(&from_node_key) {
                n1.neibs.insert(to_node_key, weight);
                Ok(())
            } else {
                Err(EdgeErr::FromNone)
            }
        } else {
            Err(EdgeErr::ToNone)
        }
    }

    /// # Error
    /// if graph doesn't contain to_node_key returns `Err(EdgeErr::ToNone)`.
    /// 
    /// if graph doesn't contain from_node_key returns `Err(EdgeErr::FromNone)`.
    pub fn remove_edge(
        &mut self,
        from_node_key: K,
        to_node_key: K,
    ) -> Result<(), EdgeErr> {
        if self.map.contains_key(&to_node_key) {
            if let Some(n1) = self.map.get_mut(&from_node_key) {
                n1.neibs.remove(&to_node_key);
                Ok(())
            } else {
                Err(EdgeErr::FromNone)
            }
        } else {
            Err(EdgeErr::ToNone)
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, T, W> {
        Iter {
            map: self.map.iter(),
        }
    }
}

impl<K: Hash + Eq + Clone, T, W> Node<K, T, W> {
    pub fn new<const N: usize>(key: K, val: T, neibs: [(K, W); N]) -> Self {
        Self {
            key,
            val,
            neibs: HashMap::from(neibs),
        }
    }

    pub const fn key(&self) -> &K {
        &self.key
    }

    pub const fn val(&self) -> &T {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut T {
        &mut self.val
    }

    pub const fn neighbors(&self) -> &HashMap<K, W> {
        &self.neibs
    }

    pub fn neighbors_as_vec(&self) -> Vec<(&K, &W)> {
        self.neibs.iter().collect()
    }

    pub fn remove_neighbor(&mut self, neib_key: K) -> Option<W> {
        self.neibs.remove(&neib_key)
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

pub struct Iter<'a, K, T, W> {
    map: hash_map::Iter<'a, K, Node<K, T, W>>,
}

impl<'a, K, T, W> Iterator for Iter<'a, K, T, W> {
    type Item = (&'a K, &'a Node<K, T, W>);

    fn next(&mut self) -> Option<Self::Item> {
        self.map.next()  
    }
}

impl<K: Hash + Eq + Clone + Debug, T: Debug, W: Debug> Display for Graph<K, T, W> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Graph {{");
        writeln!(f, "    nodes: [");
        for (k, v)  in self.iter() {
            writeln!(f, "        {{ key: {:?}, val: {:?} }},", k, v.val);
        } 
        writeln!(f, "    ]");
        writeln!(f, "\n    edges: [");
        for i in self.edges().iter() {
            writeln!(f, "        {:?},", i);
        }
        writeln!(f, "    ]");
        writeln!(f, "}}");
        Ok(())
    }
}

impl<'a, K: Display, W: Display> Display for Edge<'a, K, W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{ from: {}, to: {}, weight: {}", self.from, self.to, self.weight)
    }
}