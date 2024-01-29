
use std::collections::HashMap;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut remainders_freq: HashMap<i32, i32> = HashMap::new();
        let mut min_value = std::i32::MAX;

        // Calculate the frequency of remainders

        for &num in nums.iter() {
            let remainder = num % space;
            *remainders_freq.entry(remainder).or_insert(0) += 1;
        }

        // Find the maximum frequency of remainders

        let max_freq = remainders_freq.values().cloned().max().unwrap_or(0);

        // Find the minimum value corresponding to the maximum frequency

        for &num in nums.iter() {
            let remainder = num % space;
            if let Some(&freq) = remainders_freq.get(&remainder) {
                if freq == max_freq && num < min_value {
                    min_value = num;
                }
            }
        }

        min_value

    }
}
