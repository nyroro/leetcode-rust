
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let min_num = nums.iter().min().unwrap(); // 找到最小数

        let max_num = nums.iter().max().unwrap(); // 找到最大数

        Self::gcd(*min_num, *max_num) // 计算最大公约数

    }
    
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Self::gcd(b, a % b)
        }
    }
}
