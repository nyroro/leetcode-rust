
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];
        
        // 计算左边元素的和

        let mut sum = 0;
        for i in 0..n {
            left_sum[i] = sum;
            sum += nums[i];
        }
        
        // 计算右边元素的和

        sum = 0;
        for i in (0..n).rev() {
            right_sum[i] = sum;
            sum += nums[i];
        }
        
        // 计算左右差值

        let mut answer = vec![0; n];
        for i in 0..n {
            answer[i] = (left_sum[i] - right_sum[i]).abs();
        }
        
        answer

    }
}
