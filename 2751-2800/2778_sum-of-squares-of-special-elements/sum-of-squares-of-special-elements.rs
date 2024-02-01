
impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        // 初始化平方和为0

        let mut sum = 0;
        
        // 遍历数组并计算特殊元素的平方和

        for (i, num) in nums.iter().enumerate() {
            // 判断索引能否整除数组长度

            if (i + 1) % nums.len() == 0 {
                // 累加特殊元素的平方

                sum += num * num;
            }
        }
        
        // 返回平方和

        sum

    }
}
