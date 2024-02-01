
use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        for num in arr {
            *count_map.entry(num).or_insert(0) += 1;
        }
        
        let mut counts: Vec<i32> = count_map.values().cloned().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        
        let mut removed = 0;
        let mut set_size = 0;
        for count in counts {
            removed += count;
            set_size += 1;
            if removed >= arr.len() / 2 {
                break;
            }
        }
        
        set_size

    }
}
