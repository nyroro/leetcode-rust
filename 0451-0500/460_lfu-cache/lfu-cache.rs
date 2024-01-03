
use std::collections::{HashMap, VecDeque};

struct LFUCache {
    capacity: usize,
    vals: HashMap<i32, i32>,
    counts: HashMap<i32, i32>,
    lists: HashMap<i32, VecDeque<i32>>,
    min: i32,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            vals: HashMap::new(),
            counts: HashMap::new(),
            lists: HashMap::new(),
            min: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.vals.get(&key) {
            self.increase_count(key);
            val

        } else {
            -1

        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(val) = self.vals.get_mut(&key) {
            *val = value;
            self.increase_count(key);
            return;
        }
        if self.vals.len() >= self.capacity {
            self.evict();
        }
        self.vals.insert(key, value);
        self.counts.insert(key, 1);
        self.lists.entry(1).or_insert(VecDeque::new()).push_back(key);
        self.min = 1;
    }

    fn increase_count(&mut self, key: i32) {
        if let Some(&count) = self.counts.get(&key) {
            self.counts.insert(key, count + 1);
            self.lists.entry(count).and_modify(|list| {
                list.retain(|&k| k != key);
            });
            if let Some(list) = self.lists.get_mut(&(count + 1)) {
                list.push_back(key);
            } else {
                let mut new_list = VecDeque::new();
                new_list.push_back(key);
                self.lists.insert(count + 1, new_list);
            }
            if let Some(min_list) = self.lists.get(&self.min) {
                if min_list.is_empty() {
                    self.min += 1;
                }
            }
        }
    }

    fn evict(&mut self) {
        if let Some(min_list) = self.lists.get_mut(&self.min) {
            if let Some(&key) = min_list.front() {
                self.vals.remove(&key);
                self.counts.remove(&key);
                min_list.pop_front();
            }
        }
    }
}
