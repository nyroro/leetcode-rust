
impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        // 创建一个计数器用于统计最大公约数为 k 的子数组数量

        let mut count = 0;
        // 遍历数组 nums

        for i in 0..nums.len() {
            let mut current_gcd = 0;
            // 计算以当前元素为结尾的子数组的最大公约数

            for j in i..nums.len() {
                current_gcd = if current_gcd == 0 {
                    nums[j]
                } else {
                    Solution::gcd(current_gcd, nums[j])
                };
                // 如果当前子数组的最大公约数等于 k，则计数器加一

                if current_gcd == k {
                    count += 1;
                }
            }
        }
        // 返回计数器的值作为结果

        count

    }
    
    // 辅助函数用于计算两个数的最大公约数

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Solution::gcd(b, a % b)
        }
    }
}
