
impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        // 创建一个计数器用于统计最大公约数为 k 的子数组数量

        let mut count = 0;
        // 定义滑动窗口的左边界

        let mut left = 0;
        // 定义滑动窗口的右边界

        let mut right = 0;
        // 定义当前子数组的最大公约数

        let mut current_gcd = 0;
        
        // 辅助函数用于计算两个数的最大公约数

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a

            } else {
                gcd(b, a % b)
            }
        }
        
        // 遍历数组 nums

        while right < nums.len() {
            // 计算当前子数组的最大公约数

            current_gcd = if current_gcd == 0 {
                nums[right]
            } else {
                gcd(current_gcd, nums[right])
            };
            
            // 如果当前子数组的最大公约数等于 k，则计数器加一

            if current_gcd == k {
                count += 1;
            }
            
            // 如果当前子数组的最大公约数大于等于 k，左边界右移

            while current_gcd >= k && left < right {
                current_gcd = gcd(current_gcd, nums[left]);
                left += 1;
                if current_gcd == k {
                    count += 1;
                }
            }
            
            // 右边界右移

            right += 1;
        }
        
        // 返回计数器的值作为结果

        count

    }
}
