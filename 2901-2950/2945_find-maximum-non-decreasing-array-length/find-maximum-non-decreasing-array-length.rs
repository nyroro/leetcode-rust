
use std::cmp::max;

impl Solution {
    pub fn find_maximum_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // Create a prefix sum array to store the cumulative sum of the elements

        let mut prefix = vec![0i64; n + 1];
        for j in 1..=n {
            prefix[j] = prefix[j - 1] as i64 + nums[j - 1] as i64;
        }
        
        // Create a vector of tuples to store the maximum length after each index

        let mut max_after = vec![(1, 1); n + 1];
        
        for i in 1..=n {
            max_after[i] = max(max_after[i - 1], max_after[i]);
            
            let (cnt, ind) = max_after[i];
            let last_sum = prefix[i] - prefix[ind - 1];
            
            // Find the first valid index using binary search

            let first_valid_ind = prefix.binary_search(&(prefix[i] + last_sum)).unwrap_or_else(|x| x);
            if first_valid_ind > n {
                continue;
            }
            
            max_after[first_valid_ind] = max(max_after[first_valid_ind], (cnt + 1, i + 1));
        }
        
        return max_after[n].0;
    }
}
