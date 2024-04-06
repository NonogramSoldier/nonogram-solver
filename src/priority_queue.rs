use std::{
    cell::RefCell,
    collections::HashSet,
    hash::{BuildHasher, Hash, Hasher, RandomState},
    rc::Rc,
};

fn left_child(index: usize) -> usize {
    index * 2 + 1
}

fn right_child(index: usize) -> usize {
    index * 2 + 2
}

fn parent(index: usize) -> usize {
    if index == 0 {
        0
    } else {
        (index - 1) / 2
    }
}

pub struct PriorityQueue<K, P, S = RandomState> {
    heap: PriorityHeap<K, P>,
    set: HashSet<HashRef<K>, S>,
}

impl<K, P, S: Default> PriorityQueue<K, P, S> {
    pub fn new() -> Self {
        Self {
            heap: PriorityHeap::new(),
            set: HashSet::default(),
        }
    }
}

impl<K: Hash + Eq + Copy, P: Ord, S: BuildHasher> PriorityQueue<K, P, S> {
    pub fn insert(&mut self, key: K, priority: P) -> bool {
        let insert_value = HashRef::new(key);

        if !self.set.insert(insert_value.clone()) {
            false
        } else {
            self.heap.push(HeapNode::new(insert_value, priority));
            true
        }
    }

    pub fn pop(&mut self) -> Option<K> {
        match self.heap.pop() {
            Some(node) => {
                self.set.remove(&node.value);
                Some(node.value.refer.borrow().key)
            }
            None => None,
        }
    }

    pub fn modify(&mut self, key: K, priority: P) -> bool {
        match self.set.get(&HashRef::new(key)) {
            Some(value) => {
                let index = value.refer.borrow().index;
                self.heap.modify(index, priority);
                true
            }
            None => false,
        }
    }
}

struct PriorityHeap<K, P> {
    heap_vector: Vec<HeapNode<K, P>>,
}

impl<K, P> PriorityHeap<K, P> {
    fn new() -> Self {
        Self {
            heap_vector: Vec::new(),
        }
    }
}

impl<K, P: Ord> PriorityHeap<K, P> {
    fn push(&mut self, value: HeapNode<K, P>) {
        self.heap_vector.push(value);

        let index = self.sift_up(self.heap_vector.len() - 1);

        self.heap_vector[index].set_index(index);
    }

    fn pop(&mut self) -> Option<HeapNode<K, P>> {
        let heap_length = self.heap_vector.len();

        if heap_length <= 1 {
            return self.heap_vector.pop();
        }

        self.heap_vector.swap(0, heap_length - 1);
        let pop_value = self.heap_vector.pop();

        self.sift_down(0);

        pop_value
    }

    fn modify(&mut self, index: usize, priority: P) {
        self.heap_vector[index].set_priority(priority);

        if index == self.sift_up(index) {
            self.sift_down(index);
        }
    }

    fn swap(&mut self, index1: usize, index2: usize) {
        self.heap_vector[index1].value.set_index(index2);
        self.heap_vector[index2].value.set_index(index1);

        self.heap_vector.swap(index1, index2);
    }

    fn sift_up(&mut self, mut index: usize) -> usize {
        loop {
            let parent = parent(index);

            if self.heap_vector[index] > self.heap_vector[parent] {
                self.swap(index, parent);
            } else {
                break index;
            }

            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) -> usize {
        let heap_length = self.heap_vector.len();
        loop {
            let mut swap_index = index;
            let left_child = left_child(index);
            let right_child = right_child(index);

            if left_child < heap_length
                && self.heap_vector[swap_index] < self.heap_vector[left_child]
            {
                swap_index = left_child;
            }
            if right_child < heap_length
                && self.heap_vector[swap_index] < self.heap_vector[right_child]
            {
                swap_index = right_child;
            }

            if swap_index == index {
                break index;
            }
            self.swap(index, swap_index);
            index = swap_index;
        }
    }
}

struct HeapNode<K, P> {
    value: HashRef<K>,
    priority: P,
}

impl<K, P> HeapNode<K, P> {
    fn new(value: HashRef<K>, priority: P) -> Self {
        Self { value, priority }
    }

    fn set_priority(&mut self, priority: P) {
        self.priority = priority;
    }

    fn set_index(&self, index: usize) {
        self.value.set_index(index);
    }
}

impl<K, P: PartialEq> PartialEq for HeapNode<K, P> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<K, P: Eq> Eq for HeapNode<K, P> {}

impl<K, P: PartialOrd> PartialOrd for HeapNode<K, P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<K, P: Ord> Ord for HeapNode<K, P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

struct HashRef<K> {
    refer: Rc<RefCell<KeyIndex<K>>>,
}

impl<K> HashRef<K> {
    fn new(key: K) -> Self {
        Self {
            refer: Rc::new(RefCell::new(KeyIndex::new(key))),
        }
    }

    fn set_index(&self, index: usize) {
        self.refer.borrow_mut().set_index(index);
    }

    fn clone(&self) -> Self {
        Self {
            refer: self.refer.clone(),
        }
    }
}

impl<K: Hash> Hash for HashRef<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.refer.borrow().hash(state);
    }
}

impl<K: PartialEq> PartialEq for HashRef<K> {
    fn eq(&self, other: &Self) -> bool {
        *self.refer.borrow() == *other.refer.borrow()
    }
}

impl<K: Eq> Eq for HashRef<K> {}

struct KeyIndex<K> {
    key: K,
    index: usize,
}

impl<K> KeyIndex<K> {
    fn new(key: K) -> Self {
        Self { key, index: 0 }
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl<K: Hash> Hash for KeyIndex<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<K: PartialEq> PartialEq for KeyIndex<K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Eq> Eq for KeyIndex<K> {}