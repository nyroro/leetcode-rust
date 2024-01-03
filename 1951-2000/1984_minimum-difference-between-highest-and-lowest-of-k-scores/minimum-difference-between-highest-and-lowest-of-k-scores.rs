
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort(); // 对数组进行排序

        let mut min_diff = i32::MAX; // 初始化最小差值为最大整数值


        for i in 0..=(nums.len() - k as usize) {
            let diff = nums[i + k as usize - 1] - nums[i]; // 计算当前窗口中的最高分和最低分之间的差值

            min_diff = min_diff.min(diff); // 更新最小差值

        }

        min_diff // 返回最小差值作为结果

    }
}
