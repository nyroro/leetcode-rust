
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];
        
        left_sum[0] = nums[0];
        for i in 1..n {
            left_sum[i] = left_sum[i - 1] + nums[i];
        }
        
        right_sum[n - 1] = nums[n - 1];
        for i in (0..n-1).rev() {
            right_sum[i] = right_sum[i + 1] + nums[i];
        }
        
        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = (i as i32 * nums[i] - left_sum[i]) + (right_sum[i] - (n as i32 - i as i32 - 1) * nums[i]);
        }
        
        result

    }
}
