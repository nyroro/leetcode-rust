
impl Solution {
    pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
        let mut max_sum = 0;
        let mut current_sum = 0;
        
        for &num in nums.iter() {
            current_sum = i32::max(0, current_sum + num);
            max_sum = i32::max(max_sum, current_sum);
        }
        
        max_sum as i64

    }
}
