
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev = vec![-1; n];
        
        let mut max_len = 0;
        let mut max_index = 0;
        
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j as i32;
                }
            }
            if dp[i] > max_len {
                max_len = dp[i];
                max_index = i;
            }
        }
        
        let mut result = Vec::new();
        let mut curr = max_index as i32;
        while curr != -1 {
            result.push(nums[curr as usize]);
            curr = prev[curr as usize];
        }
        
        result

    }
}
