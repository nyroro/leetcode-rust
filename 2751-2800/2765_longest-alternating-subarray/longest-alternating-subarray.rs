
impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut max_length = -1;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[j - 1] != nums[j] + (-1_i32).pow((j - i) as u32) {
                    break;
                }
                max_length = max_length.max((j - i + 1) as i32);
            }
        }
        max_length

    }
}
