
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        
        let mut max_sum = 0;
        
        for i in 0..=n - first_len as usize {
            let first_sum = prefix_sum[i + first_len as usize] - prefix_sum[i];
            
            for j in (i + first_len as usize)..=n - second_len as usize {
                let second_sum = prefix_sum[j + second_len as usize] - prefix_sum[j];
                
                max_sum = max_sum.max(first_sum + second_sum);
            }
        }
        
        for i in 0..=n - second_len as usize {
            let second_sum = prefix_sum[i + second_len as usize] - prefix_sum[i];
            
            for j in (i + second_len as usize)..=n - first_len as usize {
                let first_sum = prefix_sum[j + first_len as usize] - prefix_sum[j];
                
                max_sum = max_sum.max(first_sum + second_sum);
            }
        }
        
        max_sum

    }
}
