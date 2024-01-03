
use std::collections::HashMap;

impl Solution {
    pub fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32 {
        let mut label_count: HashMap<i32, i32> = HashMap::new();
        let mut score = 0;
        let mut count = 0;

        let mut items: Vec<(i32, i32)> = values.into_iter().zip(labels.into_iter()).collect();
        items.sort_by(|a, b| b.0.cmp(&a.0));

        for (value, label) in items {
            if count >= num_wanted {
                break;
            }
            if *label_count.get(&label).unwrap_or(&0) < use_limit {
                score += value;
                *label_count.entry(label).or_insert(0) += 1;
                count += 1;
            }
        }

        score

    }
}
