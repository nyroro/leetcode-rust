
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut first_occurrence: HashMap<i32, usize> = HashMap::new();
        let mut last_occurrence: HashMap<i32, usize> = HashMap::new();
        let mut frequency: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if !first_occurrence.contains_key(&num) {
                first_occurrence.insert(num, i);
            }
            last_occurrence.insert(num, i);
            *frequency.entry(num).or_insert(0) += 1;
        }

        let degree = *frequency.values().max().unwrap_or(&0);
        let mut min_length = nums.len();

        for (&num, &freq) in &frequency {
            if freq == degree {
                min_length = min_length.min(last_occurrence[&num] - first_occurrence[&num] + 1);
            }
        }

        min_length as i32

    }
}
