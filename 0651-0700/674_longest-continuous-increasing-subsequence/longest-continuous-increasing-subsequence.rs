
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut start = 0;
        let mut end = 0;
        let mut max_length = 1;
        
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                end = i;
            } else {
                max_length = max_length.max(end - start + 1);
                start = i;
                end = i;
            }
        }
        
        max_length.max(end - start + 1)
    }
}
