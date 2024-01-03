
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, std::i32::MIN, std::i32::MIN];
        
        for &num in &nums {
            let mut temp = dp.clone();
            for &sum in &dp {
                let new_sum = sum + num;
                let index = new_sum as usize % 3;
                temp[index] = temp[index].max(new_sum);
            }
            dp = temp;
        }
        
        dp[0]
    }
}
