
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        let mut result = Vec::new();
        
        // 遍历数组，将出现的数字对应的索引位置的数字变为负数

        for i in 0..n {
            let index = (nums[i].abs() - 1) as usize;
            nums[index] = -nums[index].abs();
        }
        
        // 遍历数组，找到为正数的索引位置，即为缺失的数字

        for i in 0..n {
            if nums[i] > 0 {
                result.push((i + 1) as i32);
            }
        }
        
        result

    }
}
