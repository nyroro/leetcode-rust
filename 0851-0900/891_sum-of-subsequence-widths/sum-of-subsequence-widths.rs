
impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let n = nums.len();
        let mut result: i64 = 0;
        let mut pow2: Vec<i64> = vec![1; n];
        
        for i in 1..n {
            pow2[i] = pow2[i - 1] * 2 % modulo as i64;
        }
        
        let mut nums = nums;
        nums.sort();
        
        for i in 0..n {
            result = (result + nums[i] as i64 * pow2[i] - nums[n - i - 1] as i64 * pow2[i]) % modulo as i64;
        }
        
        result as i32

    }
}
