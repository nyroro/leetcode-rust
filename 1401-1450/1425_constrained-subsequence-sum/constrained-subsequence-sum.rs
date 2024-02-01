
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        let mut max_sum = nums[0];

        dp[0] = nums[0];
        let mut deque = VecDeque::new();
        deque.push_back(0);

        for i in 1..n {
            while !deque.is_empty() && (i as i32 - deque[0] as i32) > k {
                deque.pop_front();
            }

            dp[i] = max(dp[deque[0]], 0) + nums[i];
            max_sum = max(max_sum, dp[i]);

            while !deque.is_empty() && dp[i] >= dp[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(i);
        }

        max_sum

    }
}
