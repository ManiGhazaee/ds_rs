use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display, Formatter},
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<K, T, W> {
    map: HashMap<K, Node<K, T, W>>,
}

#[derive(Debug, Clone)]
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

impl<K: Debug, T, W> Graph<K, T, W>
where
    K: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.map.len() == 0
    }

    pub fn get(&self, node_key: &K) -> Option<&Node<K, T, W>> {
        self.map.get(node_key)
    }

    pub fn get_mut(&mut self, node_key: &K) -> Option<&mut Node<K, T, W>> {
        self.map.get_mut(node_key)
    }

    pub fn contains(&self, node_key: &K) -> bool {
        self.map.contains_key(node_key)
    }

    /// # Returns
    /// returns old value if map has the node.key
    /// returns None if map doesn't have the node.key
    pub fn insert(&mut self, node: Node<K, T, W>) -> Option<Node<K, T, W>> {
        self.map.insert(node.key.clone(), node)
    }

    pub fn insert_node(&mut self, key: K, val: T) {
        self.insert(Node::new(key, val));
    }

    pub fn remove(&mut self, node_key: K) -> Option<Node<K, T, W>> {
        self.map.remove(&node_key)
    }

    pub fn nodes(&self) -> Vec<&Node<K, T, W>> {
        self.map.values().into_iter().collect()
    }

    pub fn nodes_len(&self) -> usize {
        self.map.len()
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
            ret += i.1.neibs.len();
        }
        ret
    }

    /// # Error
    /// if graph doesn't contain to_node_key returns `Err(EdgeErr::ToNone)`.
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

    /// # Returns
    /// if `from_node_key`'s `Node` contains neighbor `to_node_key` returns old weight.
    ///
    /// # Error
    /// if graph doesn't contain `to_node_key` returns `Err(EdgeErr::ToNone)`.
    /// if graph doesn't contain `from_node_key` returns `Err(EdgeErr::FromNone)`.
    pub fn remove_edge(&mut self, from_node_key: K, to_node_key: K) -> Result<Option<W>, EdgeErr> {
        if self.map.contains_key(&to_node_key) {
            if let Some(n1) = self.map.get_mut(&from_node_key) {
                Ok(n1.neibs.remove(&to_node_key))
            } else {
                Err(EdgeErr::FromNone)
            }
        } else {
            Err(EdgeErr::ToNone)
        }
    }

    /// # Error
    /// if graph doesn't contain `from_node_key`'s Node, returns `Err(EdgeErr::FromNone)`.
    /// if `from_node_key`'s Node doesn't contain `to_node_key` neighbor, returns `Err(EdgeErr::ToNone)`.
    pub fn get_weight(&self, from_node_key: K, to_node_key: K) -> Result<&W, EdgeErr> {
        if let Some(n1) = self.map.get(&from_node_key) {
            match n1.neibs.get(&to_node_key) {
                Some(w) => Ok(w),
                None => Err(EdgeErr::ToNone),
            }
        } else {
            Err(EdgeErr::FromNone)
        }
    }

    /// # Error
    /// if graph doesn't contain `from_node_key`'s Node, returns `Err(EdgeErr::FromNone)`.
    /// if `from_node_key`'s Node doesn't contain `to_node_key` neighbor, returns `Err(EdgeErr::ToNone)`.
    pub fn get_weight_mut(&mut self, from_node_key: K, to_node_key: K) -> Result<&mut W, EdgeErr> {
        if let Some(n1) = self.map.get_mut(&from_node_key) {
            match n1.neibs.get_mut(&to_node_key) {
                Some(w) => Ok(w),
                None => Err(EdgeErr::ToNone),
            }
        } else {
            Err(EdgeErr::FromNone)
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, T, W> {
        Iter {
            map: self.map.iter(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, T, W> {
        IterMut {
            map: self.map.iter_mut(),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn dfs_iter<'a>(&'a self, start_node_key: &'a K) -> DfsIter<'a, K, T, W> {
        DfsIter::new(&self.map, start_node_key)
    }

    pub fn bfs_iter<'a>(&'a self, start_node_key: &'a K) -> BfsIter<'a, K, T, W> {
        BfsIter::new(&self.map, start_node_key)
    }

    fn in_outs(&self) -> HashMap<K, (isize, isize)> {
        let mut in_outs: HashMap<K, (isize /* in */, isize /* out */)> = HashMap::new();
        for (key, node) in self.iter() {
            let outs = node.neibs.len();
            if let Some(node) = in_outs.get_mut(key) {
                (*node).1 = outs as isize;
            } else {
                in_outs.insert(key.clone(), (0, outs as isize));
            }
            for (neib, _) in node.neibs.iter() {
                if let Some(neib) = in_outs.get_mut(neib) {
                    (*neib).0 += 1;
                } else {
                    in_outs.insert(neib.clone(), (1, 0));
                }
            }
        }
        in_outs
    }

    pub fn find_eulerian_path(&self) -> Option<Vec<&K>> {
        let mut in_outs = self.in_outs();
        if !self._has_eulerian_path(&in_outs) || in_outs.len() == 0 {
            return None;
        }
        let start_node = self.eulerian_path_start_node(&in_outs);
        let mut result_path = VecDeque::new();
        let mut visited_edges: HashSet<(&K, &K)> = Default::default();
        self.eulerian_path_dfs(
            &start_node.key,
            &mut in_outs,
            &mut result_path,
            &mut visited_edges,
        );
        Some(result_path.into_iter().collect())
    }

    fn eulerian_path_dfs<'a>(
        &'a self,
        key: &'a K,
        in_outs: &mut HashMap<K, (isize, isize)>,
        path: &mut VecDeque<&'a K>,
        visited_edges: &mut HashSet<(&'a K, &'a K)>,
    ) {
        while in_outs.get(&key).unwrap().1 != 0 {
            (*in_outs.get_mut(&key).unwrap()).1 -= 1;
            for (next_key, _) in self.get(&key).unwrap().neibs.iter() {
                if !visited_edges.contains(&(key, next_key)) {
                    visited_edges.insert((key, next_key));
                    self.eulerian_path_dfs(next_key, in_outs, path, visited_edges);
                    break;
                }
            }
        }
        path.push_front(key);
    }

    pub fn has_eulerian_path(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        let in_outs = self.in_outs();
        let (mut start, mut end) = (0, 0);
        for (key, _) in self.iter() {
            let (ins, outs) = in_outs.get(key).unwrap();
            if (outs - ins > 1) || (ins - outs > 1) {
                return false;
            } else if outs - ins == 1 {
                start += 1;
            } else if ins - outs == 1 {
                end += 1;
            }
        }
        (end == 0 && start == 0) || (end == 1 && start == 1)
    }

    fn _has_eulerian_path(&self, in_outs: &HashMap<K, (isize, isize)>) -> bool {
        if self.is_empty() {
            return false;
        }
        let (mut start, mut end) = (0, 0);
        for (key, _) in self.iter() {
            let (ins, outs) = in_outs.get(key).unwrap();
            if (outs - ins > 1) || (ins - outs > 1) {
                return false;
            } else if outs - ins == 1 {
                start += 1;
            } else if ins - outs == 1 {
                end += 1;
            }
        }
        (end == 0 && start == 0) || (end == 1 && start == 1)
    }

    fn eulerian_path_start_node(&self, in_outs: &HashMap<K, (isize, isize)>) -> &Node<K, T, W> {
        let mut iter = self.iter().peekable();
        let mut start_node = iter.peek().unwrap().1;
        while let Some((key, node)) = iter.next() {
            let (ins, outs) = in_outs.get(key).unwrap();
            if outs - ins == 1 {
                return node;
            }
            if *outs > 0 {
                start_node = node;
            }
        }
        return start_node;
    }
}

struct DijkstraPair<K, W>(K, W);

impl<K, W> PartialEq for DijkstraPair<K, W>
where
    W: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
impl<K, W> PartialOrd for DijkstraPair<K, W>
where
    W: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}
impl<K, W> Eq for DijkstraPair<K, W> where W: Eq {}
impl<K, W> Ord for DijkstraPair<K, W>
where
    W: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl<K: Debug, T> Graph<K, T, usize>
where
    K: Hash + Eq + Clone,
{
    pub fn dijkstra_shortest_path<'a>(
        &'a self,
        start_node_key: &'a K,
        dest_node_key: &'a K,
    ) -> Option<Vec<&'a K>>
    where
        K: Hash + Eq + Clone,
    {
        let mut prio: BinaryHeap<Reverse<DijkstraPair<&K, usize>>> = BinaryHeap::new();
        let mut dist: HashMap<&K, usize> = self.map.keys().map(|k| (k, usize::MAX)).collect();
        let mut prev: HashMap<&K, Option<&K>> = HashMap::new();
        let mut visited = HashSet::new();

        visited.insert(start_node_key);
        prio.push(Reverse(DijkstraPair(start_node_key, 0)));
        *dist.get_mut(start_node_key).unwrap() = 0;

        while let Some(Reverse(DijkstraPair(current_node_key, _current_dist))) = prio.pop() {
            let current_node = self.get(&current_node_key).unwrap();
            if current_node_key == dest_node_key {
                let mut path = vec![current_node_key];
                let mut node = current_node_key;
                while let Some(prev_node) = prev.get(node) {
                    path.push(&prev_node.unwrap());
                    node = &prev_node.unwrap();
                }
                path.reverse();
                return Some(path);
            }

            for (neib_k, neib_w) in current_node.neighbors() {
                let new_dist = neib_w + dist.get(&current_node.key).unwrap();
                if new_dist < *dist.get(neib_k).unwrap() {
                    *dist.get_mut(neib_k).unwrap() = new_dist;
                    prio.push(Reverse(DijkstraPair(neib_k, new_dist)));
                    prev.insert(neib_k, Some(current_node_key));
                }
            }
            visited.insert(current_node_key);
        }

        None
    }

    pub fn dijkstra_shortest_dist<'a>(&self, start_node_key: &'a K) -> Vec<(&K, usize)> {
        let mut prio: BinaryHeap<Reverse<DijkstraPair<&K, usize>>> = BinaryHeap::new();
        let mut dist: HashMap<&K, usize> = self.map.keys().map(|k| (k, usize::MAX)).collect();
        let mut visited = HashSet::new();

        visited.insert(start_node_key);
        prio.push(Reverse(DijkstraPair(&start_node_key, 0)));
        *dist.get_mut(start_node_key).unwrap() = 0;

        while let Some(pair) = prio.pop() {
            let current_node = self.get(&pair.0 .0).unwrap();
            for i in current_node.neighbors() {
                let neib_w = i.1;
                let neib_k = i.0;
                let new_dist = neib_w + dist.get(&current_node.key).unwrap();
                if *dist.get(neib_k).unwrap() > new_dist {
                    *dist.get_mut(neib_k).unwrap() = new_dist;
                };

                if !visited.contains(neib_k) {
                    prio.push(Reverse(DijkstraPair(neib_k, *neib_w)));
                }
            }
            visited.insert(&current_node.key);
        }

        dist.into_iter().collect()
    }
}

impl<K, T, W> Node<K, T, W>
where
    K: Hash + Eq + Clone,
{
    pub fn new(key: K, val: T) -> Self {
        Self {
            key,
            val,
            neibs: HashMap::new(),
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

impl<K, T, W> PartialEq for Node<K, T, W>
where
    K: PartialEq + Hash + Eq + Clone,
    T: PartialEq,
    W: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.val == other.val && self.neighbors() == other.neighbors()
    }
}

impl<'a, K, W> Edge<'a, K, W> {
    pub fn new(from: &'a K, to: &'a K, weight: &'a W) -> Self {
        Edge { from, to, weight }
    }
}

pub struct DfsIter<'a, K, T, W> {
    map: &'a HashMap<K, Node<K, T, W>>,
    stack: Vec<&'a Node<K, T, W>>,
    visited: HashSet<&'a K>,
}

impl<'a, K, T, W> DfsIter<'a, K, T, W>
where
    K: Hash + Eq + Clone,
{
    /// # Panics
    /// if graph doesn't contain `start_node`.
    pub fn new(map: &'a HashMap<K, Node<K, T, W>>, start_node_key: &'a K) -> Self {
        let mut visited = HashSet::new();
        visited.insert(start_node_key);
        let start_node = map.get(start_node_key).unwrap();
        let mut stack = Vec::new();
        stack.push(start_node);

        Self {
            map,
            stack,
            visited,
        }
    }
}

impl<'a, K, T, W> Iterator for DfsIter<'a, K, T, W>
where
    K: Eq + Hash + Eq + Clone,
{
    type Item = &'a Node<K, T, W>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.stack.pop()?;
        for (neighbour_key, _) in current_node.neighbors() {
            if !self.visited.contains(neighbour_key) {
                let neighbour_node = self.map.get(neighbour_key).unwrap();
                self.stack.push(neighbour_node);
                self.visited.insert(neighbour_key);
            }
        }
        Some(current_node)
    }
}

pub struct BfsIter<'a, K, T, W> {
    map: &'a HashMap<K, Node<K, T, W>>,
    queue: VecDeque<&'a Node<K, T, W>>,
    visited: HashSet<&'a K>,
}

impl<'a, K, T, W> BfsIter<'a, K, T, W>
where
    K: Hash + Eq + Clone,
{
    /// # Panics
    /// if graph doesn't contain `start_node`.
    pub fn new(map: &'a HashMap<K, Node<K, T, W>>, start_node_key: &'a K) -> Self {
        let mut visited = HashSet::new();
        visited.insert(start_node_key);
        let start_node = map.get(start_node_key).unwrap();
        let mut queue = VecDeque::new();
        queue.push_back(start_node);

        Self {
            map,
            queue,
            visited,
        }
    }
}

impl<'a, K, T, W> Iterator for BfsIter<'a, K, T, W>
where
    K: Eq + Hash + Eq + Clone,
{
    type Item = &'a Node<K, T, W>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.queue.pop_front()?;
        for (neighbour_key, _) in current_node.neighbors() {
            if !self.visited.contains(neighbour_key) {
                let neighbour_node = self.map.get(neighbour_key).unwrap();
                self.queue.push_back(neighbour_node);
                self.visited.insert(neighbour_key);
            }
        }
        Some(current_node)
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

pub struct IterMut<'a, K, T, W> {
    map: hash_map::IterMut<'a, K, Node<K, T, W>>,
}

impl<'a, K, T, W> Iterator for IterMut<'a, K, T, W> {
    type Item = (&'a K, &'a mut Node<K, T, W>);

    fn next(&mut self) -> Option<Self::Item> {
        self.map.next()
    }
}

impl<'a, K, T, W> IntoIterator for &'a Graph<K, T, W> {
    type Item = (&'a K, &'a Node<K, T, W>);
    type IntoIter = hash_map::Iter<'a, K, Node<K, T, W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a, K, T, W> IntoIterator for &'a mut Graph<K, T, W> {
    type Item = (&'a K, &'a mut Node<K, T, W>);
    type IntoIter = hash_map::IterMut<'a, K, Node<K, T, W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}

impl<K, T, W> IntoIterator for Graph<K, T, W> {
    type Item = (K, Node<K, T, W>);
    type IntoIter = hash_map::IntoIter<K, Node<K, T, W>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<K, T, W> Display for Graph<K, T, W>
where
    K: Hash + Eq + Clone + Debug,
    T: Debug,
    W: Debug + Clone,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "Graph {{")?;
        writeln!(f, "    nodes: [")?;
        for (k, v) in self.iter() {
            writeln!(f, "        {{ key: {:?}, val: {:?} }},", k, v.val)?;
        }
        writeln!(f, "    ]")?;
        writeln!(f, "\n    edges: [")?;
        for i in self.edges().iter() {
            writeln!(f, "        {:?},", i)?;
        }
        writeln!(f, "    ]")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl<K, T, W> Display for Node<K, W, T>
where
    K: Debug,
    T: Debug,
    W: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Node {{ key: {:?}, val: {:?}, neibs: {:?} }}",
            self.key, self.val, self.neibs
        )
    }
}

impl<'a, K, W> Display for Edge<'a, K, W>
where
    K: Debug,
    W: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{ from: {:?}, to: {:?}, weight: {:?} }}",
            self.from, self.to, self.weight
        )
    }
}
