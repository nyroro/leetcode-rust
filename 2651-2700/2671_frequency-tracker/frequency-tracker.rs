
use std::collections::HashMap;

struct FrequencyTracker {
    frequencies: HashMap<i32, i32>,
    frequency_count: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            frequencies: HashMap::new(),
            frequency_count: HashMap::new(),
        }
    }
    
    fn add(&mut self, number: i32) {
        let count = self.frequencies.entry(number).or_insert(0);
        *count += 1;
        
        let prev_count = *count - 1;
        if prev_count > 0 {
            let prev_freq_count = self.frequency_count.entry(prev_count).or_insert(0);
            *prev_freq_count -= 1;
            if *prev_freq_count == 0 {
                self.frequency_count.remove(&prev_count);
            }
        }
        
        let freq_count = self.frequency_count.entry(*count).or_insert(0);
        *freq_count += 1;
    }
    
    fn delete_one(&mut self, number: i32) {
        if let Some(count) = self.frequencies.get_mut(&number) {
            if *count > 0 {
                let prev_count = *count;
                *count -= 1;
                
                let prev_freq_count = self.frequency_count.entry(prev_count).or_insert(0);
                *prev_freq_count -= 1;
                if *prev_freq_count == 0 {
                    self.frequency_count.remove(&prev_count);
                }
                
                if *count > 0 {
                    let freq_count = self.frequency_count.entry(*count).or_insert(0);
                    *freq_count += 1;
                }
            }
        }
    }
    
    fn has_frequency(&self, frequency: i32) -> bool {
        self.frequency_count.contains_key(&frequency)
    }
}
