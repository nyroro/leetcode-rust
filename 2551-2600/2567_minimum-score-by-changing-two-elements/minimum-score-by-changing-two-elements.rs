


impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums;
        sorted_nums.sort();

        let min_diff1 = sorted_nums[sorted_nums.len() - 1] - sorted_nums[2];
        let min_diff2 = sorted_nums[sorted_nums.len() - 2] - sorted_nums[1];
        let min_diff3 = sorted_nums[sorted_nums.len() - 3] - sorted_nums[0];

        min_diff1.min(min_diff2).min(min_diff3)
    }
}
