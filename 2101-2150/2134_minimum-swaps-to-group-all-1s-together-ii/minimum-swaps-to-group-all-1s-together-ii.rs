
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
        
        // 将数组复制一份拼接在原数组后面，以处理循环的情况

        let nums_extended: Vec<i32> = nums.iter().chain(nums.iter()).cloned().collect();
        
        for i in 0..(2 * n) {
            if nums_extended[i] == 1 {
                current_ones += 1;
            }
            
            if i >= ones {
                if nums_extended[i - ones] == 1 {
                    current_ones -= 1;
                }
            }
            
            max_ones = max_ones.max(current_ones);
        }
        
        count = (ones - max_ones) as i32;
        
        count.min((n - ones) as i32)
    }
}
