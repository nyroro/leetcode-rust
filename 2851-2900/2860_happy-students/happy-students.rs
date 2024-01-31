
use std::collections::HashMap;



impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut counter: HashMap<i32, i32> = HashMap::new();

        for &num in &nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut selected_count = 0;
        let mut sorted_counter: Vec<_> = counter.iter().collect();
        sorted_counter.sort_by_key(|&(k, _)| k);

        for i in 0..sorted_counter.len() {
            let (num, &count) = sorted_counter[i];
            selected_count += count;
            if i + 1 < sorted_counter.len() {
                if selected_count > *num && selected_count < *sorted_counter[i + 1].0 {
                    result += 1;
                }
            } else {
                if selected_count > *num {
                    result += 1;
                }
            }
        }

        if !counter.contains_key(&0) {
            result += 1;
        }

        result

    }
}
