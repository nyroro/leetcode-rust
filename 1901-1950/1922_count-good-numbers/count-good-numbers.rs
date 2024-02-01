
// 创建一个名为Solution的结构体



impl Solution {
    // 实现count_good_numbers函数

    pub fn count_good_numbers(n: i64) -> i32 {
        // 定义模数

        let modulo: i64 = 1000000007;
        
        // 计算偶数位和奇数位的数字个数

        let even_count = n / 2;
        let odd_count = n - even_count;
        
        // 计算偶数位和奇数位的数字中好数字的个数

        let even_good_count = if even_count > 0 {
            Self::mod_pow(4, even_count as u64, modulo)
        } else {
            1

        };
        let odd_good_count = Self::mod_pow(5, odd_count as u64, modulo);
        
        // 计算总的好数字字符串的个数

        let total_good_count = (even_good_count as i64 * odd_good_count as i64) % modulo;
        
        total_good_count as i32

    }
    
    // 实现模指数函数

    fn mod_pow(mut base: i64, mut exp: u64, modulo: i64) -> i64 {
        let mut result = 1;
        base %= modulo;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulo;
            }
            exp >>= 1;
            base = (base * base) % modulo;
        }
        result

    }
}
