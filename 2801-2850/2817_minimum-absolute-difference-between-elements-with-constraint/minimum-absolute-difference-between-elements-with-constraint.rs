
use std::collections::BTreeSet;



impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
        let mut vals = BTreeSet::new();
        let mut ans = i32::MAX;
        
        for (i, &v) in nums.iter().enumerate() {
            if i >= x as usize {
                vals.insert(nums[i - x as usize]);
                let k = vals.range(..=v).next_back().map(|&x| v - x);
                if let Some(diff) = k {
                    ans = ans.min(diff);
                }
                let k = vals.range(v..).next().map(|&x| x - v);
                if let Some(diff) = k {
                    ans = ans.min(diff);
                }
            }
        }
        
        ans

    }
}
