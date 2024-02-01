


use std::collections::HashMap;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        let mut nums_mut = nums.clone();
        nums_mut.reverse();
        nums_mut.push(-1);
        nums_mut.reverse();
        
        for i in 1..nums_mut.len() {
            let mut s = 0;
            let mut ans = std::i32::MAX;
            let mut mp: HashMap<i32, i32> = HashMap::new();
            
            for j in (1..=i).rev() {
                if let Some(count) = mp.get_mut(&nums_mut[j]) {
                    s += 1;
                    if *count == 1 {
                        s += 1;
                    }
                    *count += 1;
                } else {
                    mp.insert(nums_mut[j], 1);
                }
                
                ans = ans.min(dp[j-1] + s);
            }
            
            dp[i] = ans + k;
        }
        
        dp[nums.len()]
    }
}
