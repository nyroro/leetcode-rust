


impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort(); // 对 nums 数组进行排序

        let mut operations = 0;
        
        while nums[nums.len() - 1] > 0 {
            let mut min_non_zero = 101; // 初始化为大于 nums 中可能的最大值

            for &num in &nums {
                if num > 0 && num < min_non_zero {
                    min_non_zero = num;
                }
            }
            operations += 1;
            for num in &mut nums {
                if *num > 0 {
                    *num -= min_non_zero;
                }
            }
        }
        
        operations

    }
}
