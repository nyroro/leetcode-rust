
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        if n <= 4 {
            return 0;
        }
        let mut result = i32::MAX;
        for i in 0..4 {
            result = result.min(nums[n - 4 + i] - nums[i]);
        }
        result

    }
}
