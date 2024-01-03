
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        // 定义一个辅助函数，用于判断一个数是否为质数

        fn is_prime(n: i32) -> bool {
            if n <= 1 {
                return false;
            }
            for i in 2..=(n as f64).sqrt() as i32 {
                if n % i == 0 {
                    return false;
                }
            }
            true

        }
        
        let mut count = 0;
        for num in left..=right {
            let set_bits = num.count_ones() as i32;
            if is_prime(set_bits) {
                count += 1;
            }
        }
        count

    }
}
