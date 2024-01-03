
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zero_index = 0;
        
        // 遍历数组

        for i in 0..nums.len() {
            // 如果当前元素不为0，则将其放置到非零元素应该放置的位置

            if nums[i] != 0 {
                nums[non_zero_index] = nums[i];
                non_zero_index += 1;
            }
        }
        
        // 将非零元素位置指针之后的所有元素设置为0

        for i in non_zero_index..nums.len() {
            nums[i] = 0;
        }
    }
}
