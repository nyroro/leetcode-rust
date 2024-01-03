
use std::collections::HashMap;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut freq_map = HashMap::new();
        let mut freq_set = HashMap::new();
        let mut deletions = 0;

        // Count the frequency of each character

        for c in s.chars() {
            *freq_map.entry(c).or_insert(0) += 1;
        }

        // Iterate through the frequency map

        for (_, &freq) in &freq_map {
            let mut current_freq = freq;
            while freq_set.contains_key(&current_freq) {
                deletions += 1;
                current_freq -= 1;
            }
            if current_freq > 0 {
                freq_set.insert(current_freq, true);
            }
        }

        deletions

    }
}
