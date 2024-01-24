
use std::collections::HashMap;

impl Solution {
    pub fn min_groups_for_valid_assignment(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for &num in &nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        
        let mut min_count = std::i32::MAX;
        for &count in counter.values() {
            min_count = std::cmp::min(min_count, count);
        }
        
        let mut result = std::i32::MAX;
        for i in 2..=min_count {
            let mut ret = 0;
            for &count in counter.values() {
                let a = count / i;
                let b = count % i;
                if b > a {
                    ret = -1;
                    break;
                }
                let k = count / (i + 1);
                let k = if count % (i + 1) > 0 { k + 1 } else { k };
                ret += std::cmp::max(k, 1);
            }
            if ret != -1 {
                result = std::cmp::min(result, ret);
            }
        }
        
        let mut ret = 0;
        for &count in counter.values() {
            ret += count / 2 + count % 2;
        }
        
        std::cmp::min(result, ret)
    }
}
