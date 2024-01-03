
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // 创建一个空的布尔数组

        let mut result: Vec<bool> = Vec::new();
        
        // 找到 candies 数组中的最大值

        let max = *candies.iter().max().unwrap();
        
        // 遍历 candies 数组

        for &candy in &candies {
            // 将当前孩子的糖果数量与额外糖果相加

            let total_candies = candy + extra_candies;
            
            // 检查是否大于或等于最大值

            if total_candies >= max {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        
        // 返回结果数组

        result

    }
}
