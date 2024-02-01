
use std::collections::{BTreeMap, HashMap};

struct SORTracker {
    locations: BTreeMap<i32, Vec<String>>,
    counter: HashMap<String, i32>,
}

impl SORTracker {
    fn new() -> Self {
        SORTracker {
            locations: BTreeMap::new(),
            counter: HashMap::new(),
        }
    }
    
    fn add(&mut self, name: String, score: i32) {
        let entry = self.locations.entry(score).or_insert(Vec::new());
        entry.push(name.clone());
        entry.sort();
    }
    
    fn get(&mut self) -> String {
        let count = self.counter.entry("get".to_string()).or_insert(0);
        *count += 1;
        let mut result = String::new();
        let mut current_count = 0;
        for (_, names) in self.locations.iter().rev() {
            for name in names {
                current_count += 1;
                if current_count == *count {
                    result = name.clone();
                    break;
                }
            }
            if !result.is_empty() {
                break;
            }
        }
        result

    }
}
