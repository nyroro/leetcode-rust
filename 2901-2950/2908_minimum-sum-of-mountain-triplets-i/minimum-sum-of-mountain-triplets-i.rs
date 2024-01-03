
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut min_sum = std::i32::MAX;

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                for k in (j + 1)..nums.len() {
                    if nums[i] < nums[j] && nums[k] < nums[j] {
                        let current_sum = nums[i] + nums[j] + nums[k];
                        min_sum = std::cmp::min(min_sum, current_sum);
                    }
                }
            }
        }

        if min_sum == std::i32::MAX {
            return -1;
        }

        min_sum

    }
}
