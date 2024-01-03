
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut jump = 0;
        let mut end = 0;
        let mut max_position = 0;
        
        for i in 0..n-1 {
            max_position = max_position.max(i + nums[i] as usize);
            
            if i == end {
                jump += 1;
                end = max_position;
            }
        }
        
        jump

    }
}
