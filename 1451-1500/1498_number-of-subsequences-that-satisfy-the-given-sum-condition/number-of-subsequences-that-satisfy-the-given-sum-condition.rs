
// 定义 Solution 结构体



impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
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
                right -= 1;
            } else {
                result = (result + powers[right - left]) % modulo; // 更新结果

                left += 1;
            }
        }
        
        result as i32

    }
}
