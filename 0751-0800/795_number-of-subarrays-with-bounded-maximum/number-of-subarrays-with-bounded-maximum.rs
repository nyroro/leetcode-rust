
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut count = 0;
        let mut start = -1;
        let mut end = -1;
        
        for i in 0..nums.len() {
            if nums[i] > right {
                start = i as i32;
            }
            if nums[i] >= left {
                end = i as i32;
            }
            count += end - start;
        }
        
        count

    }
}
