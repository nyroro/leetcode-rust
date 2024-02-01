
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        
        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }
        
        for i in 0..n {
            for j in (i + 2)..=n {
                let sum = prefix_sum[j] - prefix_sum[i];
                if k == 0 {
                    if sum == 0 {
                        return true;
                    }
                } else if sum % k == 0 {
                    return true;
                }
            }
        }
        
        false

    }
}
