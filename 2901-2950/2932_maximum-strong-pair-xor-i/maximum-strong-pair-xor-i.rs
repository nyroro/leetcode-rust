
impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut max_xor = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if (nums[i] - nums[j]).abs() <= std::cmp::min(nums[i], nums[j]) {
                    let current_xor = nums[i] ^ nums[j];
                    if current_xor > max_xor {
                        max_xor = current_xor;
                    }
                }
            }
        }
        max_xor

    }
}
