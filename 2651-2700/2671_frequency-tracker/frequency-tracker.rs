
use std::collections::HashMap;

struct FrequencyTracker {
    frequencies: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            frequencies: HashMap::new(),
        }
    }
    
    fn add(&mut self, number: i32) {
        let count = self.frequencies.entry(number).or_insert(0);
        *count += 1;
    }
    
    fn delete_one(&mut self, number: i32) {
        if let Some(count) = self.frequencies.get_mut(&number) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.frequencies.remove(&number);
            }
        }
    }
    
    fn has_frequency(&self, frequency: i32) -> bool {
        self.frequencies.values().any(|&count| count == frequency)
    }
}

fn main() {
    let mut frequency_tracker = FrequencyTracker::new();
    frequency_tracker.add(3);
    frequency_tracker.add(3);
    println!("{}", frequency_tracker.has_frequency(2)); // Output: true
    
    let mut frequency_tracker = FrequencyTracker::new();
    frequency_tracker.add(1);
    frequency_tracker.delete_one(1);
    println!("{}", frequency_tracker.has_frequency(1)); // Output: false
    
    let mut frequency_tracker = FrequencyTracker::new();
    println!("{}", frequency_tracker.has_frequency(2)); // Output: false

    frequency_tracker.add(3);
    println!("{}", frequency_tracker.has_frequency(1)); // Output: true

}
