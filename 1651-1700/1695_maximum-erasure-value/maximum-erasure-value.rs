
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashSet::new();
        let mut left = 0;
        let mut max_sum = 0;
        let mut curr_sum = 0;

        for right in 0..nums.len() {
            while seen.contains(&nums[right]) {
                curr_sum -= nums[left];
                seen.remove(&nums[left]);
                left += 1;
            }
            curr_sum += nums[right];
            seen.insert(nums[right]);
            max_sum = max_sum.max(curr_sum);
        }

        max_sum

    }
}
