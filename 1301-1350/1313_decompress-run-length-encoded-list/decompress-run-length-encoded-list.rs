
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        // 创建一个空的结果列表

        let mut result = Vec::new();
        
        // 遍历 nums 列表

        for i in (0..nums.len()).step_by(2) {
            // 取出一对 [freq, val]
            let freq = nums[i];
            let val = nums[i + 1];
            
            // 生成子列表并添加到 result 中

            for _ in 0..freq {
                result.push(val);
            }
        }
        
        // 返回最终的解压列表

        result

    }
}
