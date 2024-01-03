
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max_diff = -1;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] < nums[j] {
                    max_diff = max_diff.max(nums[j] - nums[i]);
                }
            }
        }
        max_diff

    }
}
