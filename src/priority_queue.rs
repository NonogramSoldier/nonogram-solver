use fxhash::FxBuildHasher;
use num_traits::Zero;
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
    ops::AddAssign,
};

macro_rules! parent {
    ($index:expr) => {
        if $index == 0 {
            None
        } else {
            Some(($index - 1) >> 1)
        }
    };
}

macro_rules! left_child {
    ($index:expr) => {
        2 * $index + 1
    };
}

macro_rules! right_child {
    ($index:expr) => {
        2 * $index + 2
    };
}

/// 'PriorityQueue' is a data structure that uses a combination of a hashmap and a binary heap.
/// You can reference the value associated with a key, and by popping from the heap, you can access the key at the top.
/// Additionally, the heap is implemented as a min-heap.
#[derive(Debug)]
pub struct PriorityQueue<K, P, S = RandomState> {
    pub heap: Vec<(K, P)>,
    pub map: HashMap<K, usize, S>,
}

pub type FxPriorityQueue<K, P> = PriorityQueue<K, P, FxBuildHasher>;

impl<K, P, S: Default> Default for PriorityQueue<K, P, S> {
    fn default() -> Self {
        Self {
            heap: Default::default(),
            map: Default::default(),
        }
    }
}

impl<K, P, S> PriorityQueue<K, P, S>
where
    K: Eq + Hash + Clone,
    P: PartialOrd,
    S: BuildHasher + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_heapify(vec: Vec<(K, P)>) -> Self {
        let length = vec.len();
        let first_index = if length == 0 { 0 } else { length / 2 - 1 };
        let mut value = Self {
            heap: vec,
            map: Default::default(),
        };

        for index in (0..=first_index).rev() {
            value.partial_heapify(length, index);
        }

        for (index, node) in value.heap.iter().enumerate() {
            value.map.insert(node.0.clone(), index);
        }

        value
    }

    pub fn insert(&mut self, key: K, priority: P) {
        if self.map.contains_key(&key) {
            let index = *self.map.get(&key).unwrap();
            self.heap[index].1 = priority;

            if index == self.sift_up(index) {
                self.sift_down(index);
            }
        } else {
            let push_index = self.heap.len();

            self.heap.push((key.clone(), priority));
            self.map.insert(key, push_index);

            self.sift_up(push_index);
        }
    }

    pub fn pop(&mut self) -> Option<(K, P)> {
        let len = self.heap.len();
        if len == 0 {
            return None;
        }

        self.map.remove(&self.heap[0].0);

        if len == 1 {
            return self.heap.pop();
        }

        self.swap_node(0, len - 1);

        let result = self.heap.pop();
        self.sift_down(0);
        result
    }

    fn partial_heapify(&mut self, length: usize, mut index: usize) {
        loop {
            let mut min_node = index;
            let left_child = left_child!(index);
            let right_child = right_child!(index);

            if left_child < length && self.heap[min_node].1 > self.heap[left_child].1 {
                min_node = left_child;
            }
            if right_child < length && self.heap[min_node].1 > self.heap[right_child].1 {
                min_node = right_child;
            }

            if min_node == index {
                break;
            }

            self.heap.swap(index, min_node);
            index = min_node;
        }
    }

    fn sift_up(&mut self, mut index: usize) -> usize {
        loop {
            match parent!(index) {
                Some(parent) => {
                    if self.heap[index].1 < self.heap[parent].1 {
                        self.swap_node(index, parent);
                        index = parent;
                    } else {
                        *(self.map.get_mut(&self.heap[index].0).unwrap()) = index;
                        break index;
                    }
                }
                None => {
                    *(self.map.get_mut(&self.heap[index].0).unwrap()) = index;
                    break index;
                }
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let mut min_node = index;
            let left_child = left_child!(index);
            let right_child = right_child!(index);

            if left_child < self.heap.len() && self.heap[min_node].1 > self.heap[left_child].1 {
                min_node = left_child;
            }
            if right_child < self.heap.len() && self.heap[min_node].1 > self.heap[right_child].1 {
                min_node = right_child;
            }

            if min_node == index {
                *(self.map.get_mut(&self.heap[index].0).unwrap()) = index;
                break;
            }

            self.swap_node(index, min_node);
            index = min_node;
        }
    }

    fn swap_node(&mut self, index1: usize, index2: usize) {
        *(self.map.get_mut(&self.heap[index2].0).unwrap()) = index1;
        self.heap.swap(index1, index2);
    }
}

impl<K, P, S> PriorityQueue<K, P, S>
where
    K: Eq + Hash + Clone,
    P: PartialOrd + AddAssign + Zero + Copy,
    S: BuildHasher + Default,
{
    pub fn add_or_insert(&mut self, key: K, priority: P) {
        match self.map.get(&key) {
            Some(index) => {
                self.heap[*index].1 += priority;
                if priority < P::zero() {
                    self.sift_up(*index);
                } else {
                    self.sift_down(*index);
                }
            }
            None => {
                let index = self.heap.len();
                self.map.insert(key.clone(), index);
                self.heap.push((key, priority));
                self.sift_up(index);
            }
        }
    }
}
