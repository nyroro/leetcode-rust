
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let ones = nums.iter().filter(|&num| *num == 1).count();
        
        if ones == 0 || ones == n {
            return 0;
        }
        
        let mut count = 0;
        let mut max_ones = 0;
        let mut current_ones = 0;
        
        for i in 0..n {
            if nums[i] == 1 {
                current_ones += 1;
            }
            
            if i >= ones {
                if nums[i - ones] == 1 {
                    current_ones -= 1;
                }
            }
            
            max_ones = max_ones.max(current_ones);
        }
        
        count = ones - max_ones;
        
        count.min(n - ones)
    }
}
