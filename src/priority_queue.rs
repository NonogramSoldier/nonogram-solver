use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
};

use fxhash::FxBuildHasher;

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
    K: Eq + Hash + Copy,
    P: Ord,
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
            value.map.insert(node.0, index);
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

            self.heap.push((key, priority));
            self.map.insert(key, push_index);

            self.sift_up(push_index);
        }
    }

    pub fn pop(&mut self) -> Option<(K, P)> {
        if self.heap.len() == 0 {
            return None;
        }
        
        self.map.remove(&self.heap[0].0);

        if self.heap.len() == 1 {
            return self.heap.pop();
        }
        
        self.swap_node(0, self.heap.len() - 1);
        
        let result = self.heap.pop();
        self.sift_down(0);
        result
    }

    fn partial_heapify(&mut self, length: usize, mut index: usize) {
        loop {
            let mut max_node = index;
            let left_child = left_child!(index);
            let right_child = right_child!(index);

            if left_child < length && self.heap[max_node].1 < self.heap[left_child].1 {
                max_node = left_child;
            }
            if right_child < length && self.heap[max_node].1 < self.heap[right_child].1 {
                max_node = right_child;
            }

            if max_node == index {
                break;
            }

            self.heap.swap(index, max_node);
            index = max_node;
        }
    }

    fn sift_up(&mut self, mut index: usize) -> usize {
        loop {
            match parent!(index) {
                Some(parent) => {
                    if self.heap[index].1 > self.heap[parent].1 {
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
            let mut max_node = index;
            let left_child = left_child!(index);
            let right_child = right_child!(index);

            if left_child < self.heap.len() && self.heap[max_node].1 < self.heap[left_child].1 {
                max_node = left_child;
            }
            if right_child < self.heap.len() && self.heap[max_node].1 < self.heap[right_child].1 {
                max_node = right_child;
            }

            if max_node == index {
                *(self.map.get_mut(&self.heap[index].0).unwrap()) = index;
                break;
            }

            self.swap_node(index, max_node);
            index = max_node;
        }
    }

    fn swap_node(&mut self, index1: usize, index2: usize) {
        *(self.map.get_mut(&self.heap[index2].0).unwrap()) = index1;
        self.heap.swap(index1, index2);
    }
}
