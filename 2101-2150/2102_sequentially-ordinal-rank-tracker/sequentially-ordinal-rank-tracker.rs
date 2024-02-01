
use std::collections::BTreeMap;

struct Location {
    name: String,
    score: i32,
}

struct SORTracker {
    locations: BTreeMap<i32, Vec<String>>,
    counter: i32,
}

impl SORTracker {
    fn new() -> Self {
        SORTracker {
            locations: BTreeMap::new(),
            counter: 0,
        }
    }
    
    fn add(&mut self, name: String, score: i32) {
        let entry = self.locations.entry(score).or_insert(Vec::new());
        entry.push(name.clone());
        entry.sort();
    }
    
    fn get(&mut self) -> String {
        self.counter += 1;
        let mut result = String::new();
        for (_, names) in self.locations.iter().rev() {
            for name in names {
                if self.counter == 1 {
                    result = name.clone();
                    break;
                }
                self.counter -= 1;
            }
            if !result.is_empty() {
                break;
            }
        }
        result

    }
}
