
// 定义 Solution 结构体



impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut result = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        nums.sort(); // 对数组进行排序
        
        let mut powers = vec![1; nums.len()]; // 初始化幂次数组

        for i in 1..nums.len() {
            powers[i] = (powers[i - 1] * 2) % modulo; // 计算幂次数组

        }
        
        while left <= right {
            if nums[left] + nums[right] > target {
                if right > 0 { // 检查 right 是否大于 0

                    right -= 1;
                } else {
                    break; // 如果 right 已经为 0，跳出循环

                }
            } else {
                result = (result + powers[right - left]) % modulo; // 更新结果

                left += 1;
            }
        }
        
        result as i32

    }
}
