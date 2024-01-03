
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max_index = 0;
        let mut max_value = nums[0];
        
        for i in 1..nums.len() {
            if nums[i] > max_value {
                max_value = nums[i];
                max_index = i;
            }
        }
        
        for i in 0..nums.len() {
            if i != max_index && nums[i] * 2 > max_value {
                return -1;
            }
        }
        
        max_index as i32

    }
}
