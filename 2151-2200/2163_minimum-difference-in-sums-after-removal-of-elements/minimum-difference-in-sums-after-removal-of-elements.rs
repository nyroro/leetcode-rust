
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut sl = 0_i64;
        let mut sr = 0_i64;
        
        for i in 0..n {
            sl += nums[i] as i64;
        }
        
        for i in 2*n..nums.len() {
            sr += nums[i] as i64;
        }
        
        let mut al: BinaryHeap<i32> = nums[..n].iter().cloned().collect();
        let mut ar: BinaryHeap<i32> = nums[2*n..].iter().map(|&x| -x).collect();
        
        let mut nums = &nums[n..2*n];
        let mut dp = vec![i64::MAX; n+1];
        dp[0] = sl;
        
        for i in 0..n {
            if nums[i] < al.peek().unwrap().clone() {
                sl += nums[i] as i64;
                sl -= al.peek().unwrap().clone() as i64;
                al.pop();
                al.push(nums[i]);
            }
            dp[i+1] = sl;
        }
        
        let mut ans = dp[n] - sr;
        
        for i in (0..n).rev() {
            if nums[i] > -ar.peek().unwrap().clone() {
                sr += nums[i] as i64;
                sr -= -ar.peek().unwrap().clone() as i64;
                ar.pop();
                ar.push(-nums[i]);
            }
            ans = ans.min(dp[i] - sr);
        }
        
        ans as i64

    }
}
