
use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new()
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.map.entry(key).or_insert(Vec::new());
        entry.push((value, timestamp));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(entry) = self.map.get(&key) {
            for i in (0..entry.len()).rev() {
                if entry[i].1 <= timestamp {
                    return entry[i].0.clone();
                }
            }
        }
        String::new()
    }
}
