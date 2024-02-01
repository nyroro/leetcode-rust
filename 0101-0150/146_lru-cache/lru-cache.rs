
use std::collections::HashMap;
use std::collections::LinkedList;

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    lru_list: LinkedList<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            lru_list: LinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            self.lru_list.remove(&key);
            self.lru_list.push_front(key);
            value

        } else {
            -1

        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.lru_list.remove(&key);
        } else if self.cache.len() >= self.capacity {
            if let Some(oldest_key) = self.lru_list.pop_back() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, value);
        self.lru_list.push_front(key);
    }
}
