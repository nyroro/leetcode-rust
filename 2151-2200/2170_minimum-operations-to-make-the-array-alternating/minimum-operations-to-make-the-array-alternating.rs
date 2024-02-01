
use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut even_counts = HashMap::new();
        let mut odd_counts = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if i % 2 == 0 {
                *even_counts.entry(num).or_insert(0) += 1;
            } else {
                *odd_counts.entry(num).or_insert(0) += 1;
            }
        }

        let mut even_sorted: Vec<_> = even_counts.into_iter().collect();
        even_sorted.sort_by_key(|&(_, count)| count);

        let mut odd_sorted: Vec<_> = odd_counts.into_iter().collect();
        odd_sorted.sort_by_key(|&(_, count)| count);

        let (max_even_count, max_even_num) = even_sorted.last().unwrap_or(&(0, &0));
        let (max_odd_count, max_odd_num) = odd_sorted.last().unwrap_or(&(0, &0);

        if max_even_num != max_odd_num {
            return (nums.len() as i32) - max_even_count - max_odd_count;
        }

        let max_even_count = max_even_count.unwrap_or(&0);
        let max_odd_count = max_odd_count.unwrap_or(&0);

        if even_sorted.len() == 1 && odd_sorted.len() == 1 {
            return (nums.len() as i32) - max(max_even_count, max_odd_count);
        } else if even_sorted.len() == 1 {
            return (nums.len() as i32) - max(max_even_count + odd_sorted[odd_sorted.len() - 2].1, max_odd_count);
        } else if odd_sorted.len() == 1 {
            return (nums.len() as i32) - max(max_even_count, max_odd_count + even_sorted[even_sorted.len() - 2].1);
        } else {
            return (nums.len() as i32) - max(max_even_count + odd_sorted[odd_sorted.len() - 2].1, max_odd_count + even_sorted[even_sorted.len() - 2].1);
        }
    }
}
