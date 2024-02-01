
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pos = vec![0; n];
        let mut neg = vec![0; n];
        let mut max_len = 0;
        
        if nums[0] > 0 {
            pos[0] = 1;
        } else if nums[0] < 0 {
            neg[0] = 1;
        } else {
            pos[0] = 0;
            neg[0] = 0;
        }
        
        max_len = max_len.max(pos[0]);
        
        for i in 1..n {
            if nums[i] > 0 {
                pos[i] = pos[i - 1] + 1;
                neg[i] = if neg[i - 1] > 0 { neg[i - 1] + 1 } else { 0 };
            } else if nums[i] < 0 {
                pos[i] = if neg[i - 1] > 0 { neg[i - 1] + 1 } else { 0 };
                neg[i] = pos[i - 1] + 1;
            } else {
                pos[i] = 0;
                neg[i] = 0;
            }
            max_len = max_len.max(pos[i]);
        }
        
        max_len

    }
}
