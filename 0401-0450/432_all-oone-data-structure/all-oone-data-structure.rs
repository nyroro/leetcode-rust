
use std::collections::{HashMap, BTreeSet};

struct AllOne {
    counts: HashMap<String, i32>,
    count_strings: BTreeSet<(i32, String)>,
}

impl AllOne {
    fn new() -> Self {
        AllOne {
            counts: HashMap::new(),
            count_strings: BTreeSet::new(),
        }
    }
    
    fn inc(&mut self, key: String) {
        let count = self.counts.entry(key.clone()).or_insert(0);
        let new_count = *count + 1;
        
        if *count > 0 {
            self.count_strings.remove(&(*count, key.clone()));
        }
        
        *count = new_count;
        self.count_strings.insert((new_count, key));
    }
    
    fn dec(&mut self, key: String) {
        if let Some(count) = self.counts.get_mut(&key) {
            let new_count = *count - 1;
            self.count_strings.remove(&(*count, key.clone()));
            
            if new_count > 0 {
                *count = new_count;
                self.count_strings.insert((new_count, key));
            } else {
                self.counts.remove(&key);
            }
        }
    }
    
    fn get_max_key(&self) -> String {
        self.count_strings.iter().rev().next().map(|(_, key)| key.clone()).unwrap_or_default()
    }
    
    fn get_min_key(&self) -> String {
        self.count_strings.iter().next().map(|(_, key)| key.clone()).unwrap_or_default()
    }
}
