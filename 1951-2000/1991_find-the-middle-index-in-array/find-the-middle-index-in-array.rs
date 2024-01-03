
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left_sum = 0;
        let mut right_sum = nums.iter().sum::<i32>();
        
        for i in 0..n {
            right_sum -= nums[i];
            
            if left_sum == right_sum {
                return i as i32;
            }
            
            left_sum += nums[i];
        }
        
        -1

    }
}
