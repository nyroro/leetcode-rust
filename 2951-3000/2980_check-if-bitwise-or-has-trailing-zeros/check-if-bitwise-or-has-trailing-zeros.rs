
impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if (nums[i] | nums[j]).trailing_zeros() > 0 {
                    return true;
                }
            }
        }
        false

    }
}
