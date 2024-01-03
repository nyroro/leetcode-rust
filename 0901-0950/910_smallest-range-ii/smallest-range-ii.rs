
impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut min_diff = nums[n - 1] - nums[0];

        for i in 0..n-1 {
            let max_val = nums[i] + k;
            let min_val = nums[i+1] - k;
            let max = max_val.max(nums[n-1] - k);
            let min = min_val.min(nums[0] + k);
            min_diff = min_diff.min(max - min);
        }

        min_diff

    }
}
