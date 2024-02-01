
impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        // 计算数组中所有元素的最大公约数

        let gcd = nums.iter().fold(0, |a, &b| Solution::gcd(a, b));
        // 如果最大公约数为 1，则返回 true，否则返回 false

        gcd == 1

    }
    
    // 辅助函数来计算最大公约数

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a

    }
}
