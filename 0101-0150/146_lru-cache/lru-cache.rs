
use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    lru_list: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            lru_list: Vec::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            self.lru_list.retain(|&x| x != key);
            self.lru_list.insert(0, key);
            value

        } else {
            -1

        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.lru_list.retain(|&x| x != key);
        } else if self.cache.len() >= self.capacity {
            if let Some(oldest_key) = self.lru_list.pop() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, value);
        self.lru_list.insert(0, key);
    }
}
