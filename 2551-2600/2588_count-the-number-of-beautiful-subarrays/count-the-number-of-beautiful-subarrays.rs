
use std::collections::HashMap;



impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut prefix_xor_count = HashMap::new();
        let mut xor = 0;
        
        prefix_xor_count.insert(0, 1);

        let mut result = 0;

        for num in nums {
            xor ^= num;
            result += *prefix_xor_count.entry(xor).or_insert(0);
            *prefix_xor_count.entry(xor).or_insert(0) += 1;
        }

        result

    }
}
