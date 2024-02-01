
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut sum_map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            for j in i+2..nums.len() {
                let sum1 = nums[i] + nums[i+1];
                let sum2 = nums[j] + nums[j+1];
                if sum1 == sum2 {
                    return true;
                }
            }
        }
        false

    }
}
