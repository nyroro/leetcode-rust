
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_sum = nums[0];
        let mut max_sum = nums[0];
        
        for i in 1..nums.len() {
            current_sum = current_sum.max(current_sum + nums[i]);
            max_sum = max_sum.max(current_sum);
        }
        
        max_sum

    }
}
