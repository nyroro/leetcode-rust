
impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        // 初始化总和为0

        let mut sum = 0;
        
        // 迭代从1到n的所有数字

        for num in 1..=n {
            // 如果数字能被3、5或7整除，则将其加到总和中

            if num % 3 == 0 || num % 5 == 0 || num % 7 == 0 {
                sum += num;
            }
        }
        
        // 返回计算得到的总和

        sum

    }
}
