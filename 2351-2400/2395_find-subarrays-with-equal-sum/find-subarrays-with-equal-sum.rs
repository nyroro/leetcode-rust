
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() - 1 {
                if nums[i] + nums[i + 1] == nums[j] + nums[j + 1] {
                    return true;
                }
            }
        }
        false

    }
}
