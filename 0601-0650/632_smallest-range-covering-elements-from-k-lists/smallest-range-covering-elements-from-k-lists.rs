
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut pointers = vec![0; k];
        let mut start = 0;
        let mut end = i32::MAX;
        
        loop {
            let mut min_val = i32::MAX;
            let mut max_val = i32::MIN;
            let mut min_index = 0;
            let mut is_end_reached = false;
            
            for i in 0..k {
                if pointers[i] >= nums[i].len() {
                    is_end_reached = true;
                    break;
                }
                
                let val = nums[i][pointers[i]];
                if val < min_val {
                    min_val = val;
                    min_index = i;
                }
                max_val = max_val.max(val);
            }
            
            if is_end_reached {
                break;
            }
            
            if max_val - min_val < end - start {
                start = min_val;
                end = max_val;
            }
            
            pointers[min_index] += 1;
        }
        
        vec![start, end]
    }
}
