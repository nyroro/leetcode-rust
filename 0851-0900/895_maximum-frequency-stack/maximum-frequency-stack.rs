
use std::collections::{HashMap, VecDeque};

struct FreqStack {
    freq_map: HashMap<i32, i32>,
    group_map: HashMap<i32, VecDeque<i32>>,
    max_freq: i32,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            freq_map: HashMap::new(),
            group_map: HashMap::new(),
            max_freq: 0,
        }
    }
    
    fn push(&mut self, val: i32) {
        let count = self.freq_map.entry(val).or_insert(0);
        *count += 1;
        let freq = *count;
        if freq > self.max_freq {
            self.max_freq = freq;
        }
        self.group_map.entry(freq).or_insert(VecDeque::new()).push_back(val);
    }
    
    fn pop(&mut self) -> i32 {
        if let Some(group) = self.group_map.get_mut(&self.max_freq) {
            if let Some(val) = group.pop_back() {
                if let Some(count) = self.freq_map.get_mut(&val) {
                    *count -= 1;
                }
                if group.is_empty() {
                    self.max_freq -= 1;
                }
                return val;
            }
        }
        0

    }
}
